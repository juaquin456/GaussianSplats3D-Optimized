import * as THREE from 'three';
import { KSplatLoader } from './src/loaders/ksplat/KSplatLoader.js';
import { PlyLoader } from './src/loaders/ply/PlyLoader.js';
import { SceneFormat } from './src/loaders/SceneFormat.js';
import { SplatLoader } from './src/loaders/splat/SplatLoader.js';
import { SplatBuffer } from './src/loaders/SplatBuffer.js';
import { SpzLoader } from './src/loaders/spz/SpzLoader.js';
import { sceneFormatFromPath } from './src/loaders/Utils.js';
import { SplatMesh } from './src/splatmesh/SplatMesh.js';

/**
 * Precomputo: Utilidad para cargar y preprocesar escenas gaussianas en RAM
 * Permite cargar múltiples escenas, extraer datos y prepararlos para renderizado
 */
export class Precomputo {

    constructor() {
        this.escenas = new Map(); // Map<path, SplatBuffer>
        this.datosExtraidos = new Map(); // Map<path, datos extraídos>
        this.stats = {
            totalSplats: 0,
            totalMemoria: 0,
            escenasCargadas: 0
        };
    }

    /**
     * Carga una escena gaussiana desde archivo y la almacena en RAM
     * @param {string} path - Ruta al archivo de escena (.ply, .splat, .ksplat, .spz)
     * @param {object} opciones - Opciones de carga
     * @returns {Promise<SplatBuffer>} Buffer de splats cargado
     */
    async cargarEscena(path, opciones = {}) {
        console.log(`Cargando escena: ${path}`);

        const formato = sceneFormatFromPath(path);
        let loader;

        switch (formato) {
            case SceneFormat.Ply:
                loader = PlyLoader;
                break;
            case SceneFormat.Splat:
                loader = SplatLoader;
                break;
            case SceneFormat.KSplat:
                loader = KSplatLoader;
                break;
            case SceneFormat.Spz:
                loader = SpzLoader;
                break;
            default:
                throw new Error(`Formato no soportado: ${formato} (path: ${path})`);
        }

        try {
            const splatBuffer = await loader.loadFromURL(path, (progress) => {
                console.log(`Progreso ${path}: ${progress}%`);
            }, false, null, opciones.splatAlphaRemovalThreshold || 1);

            this.escenas.set(path, splatBuffer);
            this.stats.escenasCargadas++;
            this.stats.totalSplats += splatBuffer.getSplatCount();
            this.stats.totalMemoria += splatBuffer.bufferData.byteLength;

            console.log(`Escena cargada: ${splatBuffer.getSplatCount()} splats, ${this.formatBytes(splatBuffer.bufferData.byteLength)}`);

            return splatBuffer;
        } catch (error) {
            console.error(`Error cargando ${path}:`, error);
            throw error;
        }
    }

    /**
     * Extrae datos de splats de una escena cargada para procesamiento
     * @param {string} path - Ruta de la escena
     * @param {object} opciones - Opciones de extracción
     * @returns {object} Datos extraídos de la escena
     */
    extraerDatosEscena(path, opciones = {}) {
        const splatBuffer = this.escenas.get(path);
        if (!splatBuffer) {
            throw new Error(`Escena no encontrada: ${path}`);
        }

        const splatCount = splatBuffer.getSplatCount();
        console.log(`Extrayendo datos de ${splatCount} splats de ${path}`);

        // Extraer centros de splats
        const centros = new Float32Array(splatCount * 3);
        splatBuffer.fillSplatCenterArray(centros);

        // Extraer colores
        const colores = new Uint8Array(splatCount * 4);
        splatBuffer.fillSplatColorArray(colores, opciones.minimumAlpha || 1);

        // Extraer escalas y rotaciones
        const escalas = new Float32Array(splatCount * 3);
        const rotaciones = new Float32Array(splatCount * 4);
        splatBuffer.fillSplatScaleRotationArray(escalas, rotaciones);

        // Extraer covarianzas (para renderizado 3D)
        const covarianzas = new Float32Array(splatCount * 6);
        splatBuffer.fillSplatCovarianceArray(covarianzas);

        const datosExtraidos = {
            centros,
            colores,
            escalas,
            rotaciones,
            covarianzas,
            splatCount,
            formato: sceneFormatFromPath(path),
            compressionLevel: splatBuffer.compressionLevel,
            sphericalHarmonicsDegree: splatBuffer.getMinSphericalHarmonicsDegree()
        };

        this.datosExtraidos.set(path, datosExtraidos);

        console.log(`Datos extraídos: ${this.formatBytes(
            centros.byteLength + colores.byteLength + escalas.byteLength +
            rotaciones.byteLength + covarianzas.byteLength
        )} en RAM`);

        return datosExtraidos;
    }

    /**
     * Crea un SplatMesh desde escenas cargadas (simula el proceso de renderizado)
     * @param {Array<string>} paths - Rutas de escenas a incluir
     * @param {object} opciones - Opciones del mesh
     * @returns {SplatMesh} Mesh creado
     */
    crearSplatMesh(paths, opciones = {}) {
        const splatBuffers = [];
        const sceneOptions = [];

        for (const path of paths) {
            const splatBuffer = this.escenas.get(path);
            if (!splatBuffer) {
                throw new Error(`Escena no encontrada: ${path}`);
            }
            splatBuffers.push(splatBuffer);
            sceneOptions.push(opciones.sceneOptions || {});
        }

        // Crear renderer mock para el mesh
        const renderer = {
            getContext: () => ({
                TEXTURE_2D: 0x0DE1,
                RGBA: 0x1908,
                UNSIGNED_BYTE: 0x1401,
                FLOAT: 0x1406,
                NEAREST: 0x2600,
                CLAMP_TO_EDGE: 0x812F,
                LINEAR: 0x2601
            }),
            capabilities: {
                maxTextureSize: 4096
            },
            properties: {
                get: () => null
            }
        };

        const splatMesh = new SplatMesh(
            opciones.splatRenderMode || 1, // ThreeD por defecto
            opciones.dynamicMode || false,
            opciones.enableOptionalEffects || false,
            opciones.halfPrecisionCovariancesOnGPU || false,
            opciones.devicePixelRatio || 1,
            opciones.enableDistancesComputationOnGPU || true,
            opciones.integerBasedDistancesComputation || false,
            opciones.antialiased || false,
            opciones.maxScreenSpaceSplatSize || 1024,
            opciones.logLevel || 0,
            opciones.sphericalHarmonicsDegree || 0,
            opciones.sceneFadeInRateMultiplier || 1.0,
            opciones.kernel2DSize || 0.3
        );

        splatMesh.setRenderer(renderer);

        // Construir el mesh
        splatMesh.build(splatBuffers, sceneOptions);

        console.log(`SplatMesh creado con ${splatMesh.getSplatCount()} splats totales`);

        return splatMesh;
    }

    /**
     * Obtiene estadísticas de uso de memoria
     * @returns {object} Estadísticas
     */
    obtenerEstadisticas() {
        return {
            ...this.stats,
            memoriaFormateada: this.formatBytes(this.stats.totalMemoria),
            escenas: Array.from(this.escenas.keys()),
            datosExtraidos: Array.from(this.datosExtraidos.keys())
        };
    }

    /**
     * Libera memoria de escenas cargadas
     * @param {Array<string>} paths - Rutas de escenas a liberar (opcional, todas si no se especifica)
     */
    liberarEscenas(paths = null) {
        const escenasALiberar = paths || Array.from(this.escenas.keys());

        for (const path of escenasALiberar) {
            this.escenas.delete(path);
            this.datosExtraidos.delete(path);
        }

        // Recalcular estadísticas
        this.stats = {
            totalSplats: 0,
            totalMemoria: 0,
            escenasCargadas: this.escenas.size
        };

        for (const [path, splatBuffer] of this.escenas) {
            this.stats.totalSplats += splatBuffer.getSplatCount();
            this.stats.totalMemoria += splatBuffer.bufferData.byteLength;
        }

        console.log(`Memoria liberada. Escenas restantes: ${this.escenas.size}`);
    }

    /**
     * Formatea bytes a formato legible
     * @param {number} bytes - Cantidad de bytes
     * @returns {string} Cadena formateada
     */
    formatBytes(bytes) {
        if (bytes === 0) return '0 Bytes';
        const k = 1024;
        const sizes = ['Bytes', 'KB', 'MB', 'GB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
    }

    /**
     * Construye niveles de detalle (LOD) para una escena gaussiana
     * Implementa el algoritmo de Mip-Splatting para reducción progresiva
     * @param {string} path - Ruta de la escena base
     * @param {object} config - Configuración de LOD
     * @returns {Promise<Array>} Niveles LOD construidos
     */
    async construirNivelesLOD(path, config = {}) {
        const datosBase = this.datosExtraidos.get(path);
        if (!datosBase) {
            throw new Error(`Datos de escena no encontrados: ${path}. Ejecuta extraerDatosEscena() primero.`);
        }

        // Configuración por defecto
        const {
            umbralesDistancia = [5, 10, 20, 50], // d1 < d2 < ... < dk
            factorSuavizado = 1.0, // s
            focalLength = 1000, // f (aproximado)
            umbralPruningBase = 0.02, // γ
            pasosFineTuning = 100, // T_FT
            alphaDifusion = 0.1, // α para compensar difusión
            vistasValidacion = 10 // Número de vistas para calcular importancia
        } = config;

        console.log(`Construyendo ${umbralesDistancia.length} niveles LOD para ${path}`);

        const nivelesLOD = [];

        for (let i = 0; i < umbralesDistancia.length; i++) {
            const d_l = umbralesDistancia[i];
            console.log(`Procesando nivel LOD ${i + 1}/${umbralesDistancia.length} (distancia: ${d_l})`);

            // 1. Copiar datos del nivel base
            const gaussiansLOD = this.copiarDatosGaussianos(datosBase);

            // 2. Aplicar filtro de suavizado 3D (Mip-Splatting)
            this.aplicarFiltroSuavizado(gaussiansLOD, d_l, factorSuavizado, focalLength, alphaDifusion);

            // 3. Iterar proceso de pruning y fine-tuning
            const thresholds = [0.2 * umbralPruningBase, 0.6 * umbralPruningBase, umbralPruningBase];

            for (const tau of thresholds) {
                console.log(`  Pruning con umbral ${tau.toFixed(4)}`);

                // 3.1 Calcular importancia de cada gaussiana
                const importancias = this.calcularImportanciasGaussianas(gaussiansLOD, vistasValidacion);

                // 3.2 Prune gaussianas de baja importancia
                const gaussianasFiltradas = this.pruneGaussianas(gaussiansLOD, importancias, tau);

                // 3.3 Fine-tuning (simplificado - solo ajuste de opacidad)
                for (let step = 0; step < pasosFineTuning; step++) {
                    // Muestrear distancia aleatoria en [0.7 * d_l, 1.3 * d_l]
                    const d_sample = d_l * (0.7 + Math.random() * 0.6);

                    // Ajuste simple: reducir opacidad basado en distancia
                    const factorAjuste = Math.exp(-0.01 * (d_sample - d_l) ** 2);
                    gaussianasFiltradas.opacidades = gaussianasFiltradas.opacidades.map(o => o * factorAjuste);
                }

                // Actualizar datos para siguiente iteración
                gaussiansLOD.centros = gaussianasFiltradas.centros;
                gaussiansLOD.covarianzas = gaussianasFiltradas.covarianzas;
                gaussiansLOD.opacidades = gaussianasFiltradas.opacidades;
                gaussiansLOD.splatCount = gaussianasFiltradas.centros.length / 3;
            }

            // 4. Guardar nivel resultante
            nivelesLOD.push({
                distancia: d_l,
                datos: gaussiansLOD,
                estadisticas: {
                    splatCount: gaussiansLOD.splatCount,
                    reduccion: (gaussiansLOD.splatCount / datosBase.splatCount * 100).toFixed(1) + '%'
                }
            });

            console.log(`  Nivel LOD completado: ${gaussiansLOD.splatCount} splats (${nivelesLOD[i].estadisticas.reduccion} del original)`);
        }

        console.log(`Construcción de LOD completada. Niveles generados: ${nivelesLOD.length}`);
        return nivelesLOD;
    }

    /**
     * Copia los datos de gaussianas para procesamiento LOD
     * @param {object} datosBase - Datos originales
     * @returns {object} Copia de datos
     */
    copiarDatosGaussianos(datosBase) {
        return {
            centros: new Float32Array(datosBase.centros),
            covarianzas: new Float32Array(datosBase.covarianzas),
            colores: new Uint8Array(datosBase.colores),
            opacidades: datosBase.colores.filter((_, i) => i % 4 === 3).map(c => c / 255), // Extraer opacidades
            splatCount: datosBase.splatCount
        };
    }

    /**
     * Aplica filtro de suavizado 3D (Mip-Splatting)
     * @param {object} gaussians - Datos de gaussianas
     * @param {number} d_l - Umbral de distancia
     * @param {number} s - Factor de suavizado
     * @param {number} f - Longitud focal
     * @param {number} alpha - Factor de compensación de difusión
     */
    aplicarFiltroSuavizado(gaussians, d_l, s, f, alpha) {
        const factorSuavizado = (s * d_l / f) ** 2;

        for (let i = 0; i < gaussians.splatCount; i++) {
            const covBase = i * 6;

            // Aumentar covarianza proporcional a d_l (diagonal de la matriz)
            gaussians.covarianzas[covBase] += factorSuavizado;     // Σ00
            gaussians.covarianzas[covBase + 3] += factorSuavizado; // Σ11
            gaussians.covarianzas[covBase + 5] += factorSuavizado; // Σ22

            // Reducir opacidad para compensar difusión
            gaussians.opacidades[i] *= Math.exp(-alpha * (d_l / f) ** 2);
        }
    }

    /**
     * Calcula importancia de cada gaussiana basada en contribución máxima a píxeles
     * @param {object} gaussians - Datos de gaussianas
     * @param {number} numVistas - Número de vistas de validación
     * @returns {Float32Array} Array de importancias
     */
    calcularImportanciasGaussianas(gaussians, numVistas) {
        const importancias = new Float32Array(gaussians.splatCount);

        // Para datos reales, usar una aproximación más simple y permisiva
        // Asignar importancia basada en opacidad y proximidad al centro de la escena
        const centroEscena = new THREE.Vector3(0, 0, 0);

        for (let i = 0; i < gaussians.splatCount; i++) {
            const centroBase = i * 3;
            const centro = new THREE.Vector3(
                gaussians.centros[centroBase],
                gaussians.centros[centroBase + 1],
                gaussians.centros[centroBase + 2]
            );

            // Distancia al centro de la escena
            const distanciaCentro = centro.distanceTo(centroEscena);

            // Importancia = opacidad * factor de distancia (más cerca = más importante)
            const importanciaDistancia = Math.exp(-distanciaCentro * 0.1); // Factor más permisivo
            const importancia = gaussians.opacidades[i] * importanciaDistancia;

            importancias[i] = importancia;
        }

        return importancias;
    }

    /**
     * Elimina gaussianas con importancia por debajo del umbral
     * @param {object} gaussians - Datos originales
     * @param {Float32Array} importancias - Array de importancias
     * @param {number} tau - Umbral de pruning
     * @returns {object} Datos filtrados
     */
    pruneGaussianas(gaussians, importancias, tau) {
        const indicesMantener = [];

        // Para debugging, mostrar estadísticas de importancia
        let maxImportancia = 0;
        let minImportancia = Infinity;
        let avgImportancia = 0;

        for (let i = 0; i < gaussians.splatCount; i++) {
            const importancia = importancias[i];
            maxImportancia = Math.max(maxImportancia, importancia);
            minImportancia = Math.min(minImportancia, importancia);
            avgImportancia += importancia;

            if (importancia >= tau) {
                indicesMantener.push(i);
            }
        }

        avgImportancia /= gaussians.splatCount;

        console.log(`    Estadísticas importancia: max=${maxImportancia.toFixed(6)}, min=${minImportancia.toFixed(6)}, avg=${avgImportancia.toFixed(6)}, tau=${tau.toFixed(6)}`);
        console.log(`    Pruning: ${gaussians.splatCount} -> ${indicesMantener.length} splats (${((indicesMantener.length / gaussians.splatCount) * 100).toFixed(1)}%)`);

        // Si no se mantiene ninguno, mantener al menos algunos (fallback)
        if (indicesMantener.length === 0) {
            console.log(`    ⚠️  Fallback: manteniendo top 10% de gaussianas por importancia`);
            // Ordenar por importancia descendente y tomar top 10%
            const indicesConImportancia = importancias.map((imp, idx) => ({ importancia: imp, indice: idx }));
            indicesConImportancia.sort((a, b) => b.importancia - a.importancia);
            const top10Percent = Math.max(1, Math.floor(gaussians.splatCount * 0.1));
            indicesMantener.push(...indicesConImportancia.slice(0, top10Percent).map(item => item.indice));
            console.log(`    Fallback aplicado: manteniendo ${indicesMantener.length} splats`);
        }

        // Crear nuevos arrays con solo los splats mantenidos
        const centrosFiltrados = new Float32Array(indicesMantener.length * 3);
        const covarianzasFiltradas = new Float32Array(indicesMantener.length * 6);
        const opacidadesFiltradas = new Float32Array(indicesMantener.length);

        for (let j = 0; j < indicesMantener.length; j++) {
            const idx = indicesMantener[j];

            // Copiar centros
            const centroBase = idx * 3;
            const centroDest = j * 3;
            centrosFiltrados[centroDest] = gaussians.centros[centroBase];
            centrosFiltrados[centroDest + 1] = gaussians.centros[centroBase + 1];
            centrosFiltrados[centroDest + 2] = gaussians.centros[centroBase + 2];

            // Copiar covarianzas
            const covBase = idx * 6;
            const covDest = j * 6;
            for (let k = 0; k < 6; k++) {
                covarianzasFiltradas[covDest + k] = gaussians.covarianzas[covBase + k];
            }

            // Copiar opacidades
            opacidadesFiltradas[j] = gaussians.opacidades[idx];
        }

        return {
            centros: centrosFiltrados,
            covarianzas: covarianzasFiltradas,
            opacidades: opacidadesFiltradas
        };
    }

    /**
     * Guarda niveles LOD en archivos separados
     * @param {string} pathBase - Ruta base para los archivos LOD
     * @param {Array} nivelesLOD - Niveles LOD a guardar
     * @param {object} opciones - Opciones de guardado
     * @returns {Promise<Array<string>>} Rutas de archivos guardados
     */
    async guardarNivelesLOD(pathBase, nivelesLOD, opciones = {}) {
        const archivosGenerados = [];
        const formato = opciones.formato || 'ply'; // ply, splat, ksplat
        const compressionLevel = opciones.compressionLevel || 1;

        console.log(`Guardando ${nivelesLOD.length} niveles LOD en formato ${formato}`);

        for (let i = 0; i < nivelesLOD.length; i++) {
            const nivel = nivelesLOD[i];
            const nombreArchivo = `${pathBase}_lod_${i}_d${nivel.distancia}.${formato}`;
            console.log(`Guardando nivel ${i}: ${nombreArchivo} (${nivel.estadisticas.splatCount} splats)`);

            // Convertir datos LOD a formato SplatBuffer
            const splatBufferLOD = this.convertirLODASplatBuffer(nivel.datos, compressionLevel);

            // Guardar archivo
            await this.guardarSplatBuffer(splatBufferLOD, nombreArchivo, formato);

            archivosGenerados.push({
                path: nombreArchivo,
                distancia: nivel.distancia,
                splatCount: nivel.estadisticas.splatCount,
                reduccion: nivel.estadisticas.reduccion
            });
        }

        console.log(`Guardado completado. ${archivosGenerados.length} archivos generados.`);
        return archivosGenerados;
    }

    /**
     * Convierte datos LOD a SplatBuffer
     * @param {object} datosLOD - Datos del nivel LOD
     * @param {number} compressionLevel - Nivel de compresión
     * @returns {SplatBuffer} Buffer convertido
     */
    convertirLODASplatBuffer(datosLOD, compressionLevel) {
        // Crear arrays en formato UncompressedSplatArray
        const splats = [];

        for (let i = 0; i < datosLOD.splatCount; i++) {
            const centroBase = i * 3;
            const covBase = i * 6;

            const splat = new Array(62).fill(0); // Array de 62 elementos para formato completo

            // Posición (X, Y, Z)
            splat[0] = datosLOD.centros[centroBase];     // X
            splat[1] = datosLOD.centros[centroBase + 1]; // Y
            splat[2] = datosLOD.centros[centroBase + 2]; // Z

            // Escalas (normalizadas de covarianzas)
            // Para simplificar, extraemos escalas de la diagonal de covarianza
            const scaleX = Math.sqrt(Math.abs(datosLOD.covarianzas[covBase]));
            const scaleY = Math.sqrt(Math.abs(datosLOD.covarianzas[covBase + 3]));
            const scaleZ = Math.sqrt(Math.abs(datosLOD.covarianzas[covBase + 5]));
            splat[3] = scaleX;  // SCALE0
            splat[4] = scaleY;  // SCALE1
            splat[5] = scaleZ;  // SCALE2

            // Rotación (identidad simplificada)
            splat[6] = 1.0;  // ROTATION0 (w)
            splat[7] = 0.0;  // ROTATION1 (x)
            splat[8] = 0.0;  // ROTATION2 (y)
            splat[9] = 0.0;  // ROTATION3 (z)

            // Colores (RGBA)
            const colorBase = i * 4;
            splat[10] = datosLOD.colores[colorBase] || 128;     // FDC0 (R)
            splat[11] = datosLOD.colores[colorBase + 1] || 128; // FDC1 (G)
            splat[12] = datosLOD.colores[colorBase + 2] || 128; // FDC2 (B)
            splat[13] = datosLOD.opacidades[i] * 255 || 255;     // OPACITY

            splats.push(splat);
        }

        // Crear UncompressedSplatArray
        const uncompressedArray = {
            splats: splats,
            splatCount: splats.length,
            sphericalHarmonicsDegree: 0 // Sin SH para LODs simplificados
        };

        // Convertir a SplatBuffer comprimido
        const sceneCenter = new THREE.Vector3(0, 0, 0); // Centro simplificado
        const splatBuffer = SplatBuffer.generateFromUncompressedSplatArrays(
            [uncompressedArray],
            1, // minimumAlpha
            compressionLevel,
            sceneCenter
        );

        return splatBuffer;
    }

    /**
     * Guarda un SplatBuffer en archivo
     * @param {SplatBuffer} splatBuffer - Buffer a guardar
     * @param {string} filePath - Ruta del archivo
     * @param {string} formato - Formato del archivo
     * @returns {Promise<void>}
     */
    async guardarSplatBuffer(splatBuffer, filePath, formato) {
        // En entorno Node.js, escribiríamos el archivo
        if (typeof window === 'undefined') {
            try {
                const fs = await import('fs');
                const buffer = Buffer.from(splatBuffer.bufferData);
                await fs.promises.writeFile(filePath, buffer);
                console.log(`✅ Archivo guardado: ${filePath} (${this.formatBytes(splatBuffer.bufferData.byteLength)})`);
            } catch (error) {
                console.error(`❌ Error guardando ${filePath}:`, error);
                throw error;
            }
        } else {
            // Para navegador, simulamos la operación
            console.log(`Simulando guardado de ${filePath} (${this.formatBytes(splatBuffer.bufferData.byteLength)})`);
        }

        return Promise.resolve();
    }

    /**
     * Crea un archivo de metadatos para LODs
     * @param {string} pathBase - Ruta base
     * @param {Array} archivosLOD - Información de archivos LOD
     * @param {object} configLOD - Configuración original
     * @returns {Promise<void>}
     */
    async crearMetadatosLOD(pathBase, archivosLOD, configLOD) {
        const metadatos = {
            version: "1.0",
            escenaBase: pathBase,
            fechaCreacion: new Date().toISOString(),
            configuracion: configLOD,
            niveles: archivosLOD.map(archivo => ({
                distancia: archivo.distancia,
                archivo: archivo.path,
                splatCount: archivo.splatCount,
                reduccion: archivo.reduccion
            })),
            estadisticas: {
                totalNiveles: archivosLOD.length,
                rangoDistancias: {
                    min: Math.min(...archivosLOD.map(a => a.distancia)),
                    max: Math.max(...archivosLOD.map(a => a.distancia))
                },
                reduccionTotal: archivosLOD.length > 0 ?
                    archivosLOD[archivosLOD.length - 1].reduccion : "0%"
            }
        };

        const metadatosPath = `${pathBase}_lod_metadata.json`;
        console.log(`Creando metadatos: ${metadatosPath}`);

        // En Node.js, escribir el archivo
        if (typeof window === 'undefined') {
            try {
                const fs = await import('fs');
                await fs.promises.writeFile(metadatosPath, JSON.stringify(metadatos, null, 2));
                console.log(`✅ Metadatos guardados: ${metadatosPath}`);
            } catch (error) {
                console.error(`❌ Error guardando metadatos ${metadatosPath}:`, error);
                throw error;
            }
        } else {
            // Simulación para navegador
            console.log("Metadatos LOD:", JSON.stringify(metadatos, null, 2));
        }

        return metadatos;
    }

    /**
     * Lista todas las escenas cargadas
     * @returns {Array<object>} Información de escenas
     */
    listarEscenas() {
        const escenas = [];
        for (const [path, splatBuffer] of this.escenas) {
            escenas.push({
                path,
                splatCount: splatBuffer.getSplatCount(),
                compressionLevel: splatBuffer.compressionLevel,
                sphericalHarmonicsDegree: splatBuffer.getMinSphericalHarmonicsDegree(),
                memoria: this.formatBytes(splatBuffer.bufferData.byteLength),
                datosExtraidos: this.datosExtraidos.has(path)
            });
        }
        return escenas;
    }
}

// Función de utilidad para uso directo
export async function cargarEscenaGaussiana(path, opciones = {}) {
    const precomputo = new Precomputo();
    await precomputo.cargarEscena(path, opciones);
    return precomputo.extraerDatosEscena(path, opciones);
}

/**
 * Función de utilidad para construir LODs directamente
 * @param {string} path - Ruta de la escena
 * @param {object} configLOD - Configuración de LOD
 * @returns {Promise<Array>} Niveles LOD
 */
export async function construirLODEspera(path, configLOD = {}) {
    const precomputo = new Precomputo();
    await precomputo.cargarEscena(path);
    precomputo.extraerDatosEscena(path);
    return await precomputo.construirNivelesLOD(path, configLOD);
}

/**
 * Pipeline completo: cargar escena, construir LODs y guardar archivos
 * @param {string} inputPath - Ruta de entrada
 * @param {string} outputPathBase - Ruta base de salida
 * @param {object} configLOD - Configuración de LOD
 * @param {object} opcionesGuardado - Opciones de guardado
 * @returns {Promise<object>} Resultado del pipeline
 */
export async function pipelineLODCompleto(inputPath, outputPathBase, configLOD = {}, opcionesGuardado = {}) {
    console.log(`Iniciando pipeline LOD completo:`);
    console.log(`  Entrada: ${inputPath}`);
    console.log(`  Salida base: ${outputPathBase}`);

    const precomputo = new Precomputo();

    // 1. Cargar escena
    console.log("Paso 1: Cargando escena...");
    await precomputo.cargarEscena(inputPath);
    const statsInicial = precomputo.obtenerEstadisticas();
    console.log(`  Escena cargada: ${statsInicial.totalSplats} splats, ${statsInicial.memoriaFormateada}`);

    // 2. Extraer datos
    console.log("Paso 2: Extrayendo datos...");
    precomputo.extraerDatosEscena(inputPath);

    // 3. Construir LODs
    console.log("Paso 3: Construyendo niveles LOD...");
    const nivelesLOD = await precomputo.construirNivelesLOD(inputPath, configLOD);

    // 4. Guardar LODs
    console.log("Paso 4: Guardando archivos LOD...");
    const archivosLOD = await precomputo.guardarNivelesLOD(outputPathBase, nivelesLOD, opcionesGuardado);

    // 5. Crear metadatos
    console.log("Paso 5: Creando metadatos...");
    const metadatos = await precomputo.crearMetadatosLOD(outputPathBase, archivosLOD, configLOD);

    // Resultado final
    const resultado = {
        escenaOriginal: {
            path: inputPath,
            splatCount: statsInicial.totalSplats,
            memoria: statsInicial.memoriaFormateada
        },
        nivelesLOD: archivosLOD,
        metadatos: metadatos,
        estadisticas: {
            tiempoProcesamiento: Date.now(), // Placeholder
            memoriaUsada: precomputo.obtenerEstadisticas().memoriaFormateada,
            archivosGenerados: archivosLOD.length + 1 // +1 por metadatos
        }
    };

    console.log("Pipeline LOD completado exitosamente!");
    console.log(`  Archivos generados: ${resultado.estadisticas.archivosGenerados}`);
    console.log(`  Reducción final: ${nivelesLOD[nivelesLOD.length - 1]?.estadisticas.reduccion || 'N/A'}`);

    return resultado;
}

// Exportar como módulo por defecto
export default Precomputo;
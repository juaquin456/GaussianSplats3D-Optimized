import { pipelineLODCompleto } from './precomputo.js';

async function ejecutarPrueba() {
    try {
        console.log('🚀 Ejecutando prueba del sistema LOD...\n');

        const resultado = await pipelineLODCompleto(
            "/home/juaquin-remon/Documents/projects/GaussianSplats3D/build/demo/assets/data/bonsai/bonsai.ksplat",
            "/home/juaquin-remon/Documents/projects/GaussianSplats3D/build/demo/assets/data/bonsai/bonsai_lod",
            {
                umbralesDistancia: [5, 10, 20, 50],
                umbralPruningBase: 0.1 // Umbral más alto para permitir pruning significativo
            }
        );

        console.log('\n✅ Prueba completada exitosamente!');
        console.log('Resultado:', resultado);

    } catch (error) {
        console.error('❌ Error en la prueba:', error);
        console.error('Stack trace:', error.stack);
    }
}

ejecutarPrueba();

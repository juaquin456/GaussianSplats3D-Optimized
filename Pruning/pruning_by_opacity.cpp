#include <iostream>
#include <vector>
#include <string>
#include <cmath>
#include <algorithm>
#include <filesystem>
#include "happly.h"


struct Gaussian {
    float x, y, z;
    float nxx, ny, nz;
    float f_dc_0, f_dc_1, f_dc_2;
    std::vector<float> f_rest; 
    float opacity_logit;
    float scale_0, scale_1, scale_2;
    float rot_0, rot_1, rot_2, rot_3;
    float actual_opacity; 
};

void write_pruned_ply(
    const std::string& path,
    const std::vector<Gaussian>& gaussians,
    size_t start_index,
    size_t count)
{
    happly::PLYData plyOut;
    plyOut.addElement("vertex", count);

    std::vector<float> x, y, z, nxx, ny, nz, opacity, scale_0, scale_1, scale_2, rot_0, rot_1, rot_2, rot_3;
    std::vector<float> f_dc_0, f_dc_1, f_dc_2;
    std::vector<std::vector<float>> f_rest_all(45);

    for (size_t i = 0; i < count; ++i) {
        const auto& g = gaussians[start_index + i];
        
        x.push_back(g.x); y.push_back(g.y); z.push_back(g.z);
        nxx.push_back(g.nxx); ny.push_back(g.ny); nz.push_back(g.nz);
        f_dc_0.push_back(g.f_dc_0); f_dc_1.push_back(g.f_dc_1); f_dc_2.push_back(g.f_dc_2);
        opacity.push_back(g.opacity_logit); 

        scale_0.push_back(g.scale_0); scale_1.push_back(g.scale_1); scale_2.push_back(g.scale_2);
        rot_0.push_back(g.rot_0); rot_1.push_back(g.rot_1); 
        rot_2.push_back(g.rot_2); rot_3.push_back(g.rot_3);
        for(int j=0; j<45; ++j) {
            f_rest_all[j].push_back(g.f_rest[j]);
        }
    }

    plyOut.getElement("vertex").addProperty<float>("x", x);
    plyOut.getElement("vertex").addProperty<float>("y", y);
    plyOut.getElement("vertex").addProperty<float>("z", z);
    plyOut.getElement("vertex").addProperty<float>("nxx", nxx);
    plyOut.getElement("vertex").addProperty<float>("ny", ny);
    plyOut.getElement("vertex").addProperty<float>("nz", nz);
    plyOut.getElement("vertex").addProperty<float>("f_dc_0", f_dc_0);
    plyOut.getElement("vertex").addProperty<float>("f_dc_1", f_dc_1);
    plyOut.getElement("vertex").addProperty<float>("f_dc_2", f_dc_2);
    for (int i = 0; i < 45; ++i) {
        plyOut.getElement("vertex").addProperty<float>("f_rest_" + std::to_string(i), f_rest_all[i]);
    }
    plyOut.getElement("vertex").addProperty<float>("opacity", opacity);
    plyOut.getElement("vertex").addProperty<float>("scale_0", scale_0);
    plyOut.getElement("vertex").addProperty<float>("scale_1", scale_1);
    plyOut.getElement("vertex").addProperty<float>("scale_2", scale_2);
    plyOut.getElement("vertex").addProperty<float>("rot_0", rot_0);
    plyOut.getElement("vertex").addProperty<float>("rot_1", rot_1);
    plyOut.getElement("vertex").addProperty<float>("rot_2", rot_2);
    plyOut.getElement("vertex").addProperty<float>("rot_3", rot_3);

    plyOut.write(path, happly::DataFormat::Binary);
    std::cout << " -> Escena guardada en: " << path << std::endl;
}

int main(int argc, char** argv) {
    if (argc != 3) {
        std::cerr << "Usage: ./prune_opacity <input.ply> <output_directory>" << std::endl;
        return 1;
    }

    std::filesystem::path input_path(argv[1]);
    std::filesystem::path output_dir = argv[2];
    std::filesystem::create_directories(output_dir);

    try {
        std::cout << "Cargando fichero PLY: " << input_path.string() << "..." << std::endl;
        happly::PLYData plyIn(input_path.string());

        auto x = plyIn.getElement("vertex").getProperty<float>("x");
        auto y = plyIn.getElement("vertex").getProperty<float>("y");
        auto z = plyIn.getElement("vertex").getProperty<float>("z");
        auto nxx = plyIn.getElement("vertex").getProperty<float>("nxx");
        auto ny = plyIn.getElement("vertex").getProperty<float>("ny");
        auto nz = plyIn.getElement("vertex").getProperty<float>("nz");
        auto f_dc_0 = plyIn.getElement("vertex").getProperty<float>("f_dc_0");
        auto f_dc_1 = plyIn.getElement("vertex").getProperty<float>("f_dc_1");
        auto f_dc_2 = plyIn.getElement("vertex").getProperty<float>("f_dc_2");
        auto opacity = plyIn.getElement("vertex").getProperty<float>("opacity");
        auto scale_0 = plyIn.getElement("vertex").getProperty<float>("scale_0");
        auto scale_1 = plyIn.getElement("vertex").getProperty<float>("scale_1");
        auto scale_2 = plyIn.getElement("vertex").getProperty<float>("scale_2");
        auto rot_0 = plyIn.getElement("vertex").getProperty<float>("rot_0");
        auto rot_1 = plyIn.getElement("vertex").getProperty<float>("rot_1");
        auto rot_2 = plyIn.getElement("vertex").getProperty<float>("rot_2");
        auto rot_3 = plyIn.getElement("vertex").getProperty<float>("rot_3");
        
        std::vector<std::vector<float>> f_rest_all;
        for (int i = 0; i < 45; ++i) {
            f_rest_all.push_back(plyIn.getElement("vertex").getProperty<float>("f_rest_" + std::to_string(i)));
        }

        size_t original_count = x.size();
        std::cout << "Número original de gaussianas: " << original_count << std::endl;

        std::vector<Gaussian> all_gaussians;
        all_gaussians.reserve(original_count);

        for (size_t i = 0; i < original_count; ++i) {
            Gaussian g;
            g.x = x[i]; g.y = y[i]; g.z = z[i];
            g.nxx = nxx[i]; g.ny = ny[i]; g.nz = nz[i];
            g.f_dc_0 = f_dc_0[i]; g.f_dc_1 = f_dc_1[i]; g.f_dc_2 = f_dc_2[i];
            g.opacity_logit = opacity[i];
            g.scale_0 = scale_0[i]; g.scale_1 = scale_1[i]; g.scale_2 = scale_2[i];
            g.rot_0 = rot_0[i]; g.rot_1 = rot_1[i]; g.rot_2 = rot_2[i]; g.rot_3 = rot_3[i];
            g.f_rest.resize(45);
            for(int j=0; j<45; ++j) g.f_rest[j] = f_rest_all[j][i];
            
            // Calcular la opacidad real usando la función sigmoide
            g.actual_opacity = 1.0f / (1.0f + std::exp(-g.opacity_logit));
            all_gaussians.push_back(g);
        }
        
        // Ordenar el vector por opacidad real, de menor a mayor
        std::sort(all_gaussians.begin(), all_gaussians.end(), [](const Gaussian& a, const Gaussian& b) {
            return a.actual_opacity < b.actual_opacity;
        });

        std::vector<int> percentages_to_prune = {10, 20, 30, 40, 50};
        std::string base_name = input_path.stem().string();

        for (int percentage : percentages_to_prune) {
            size_t num_to_remove = original_count * percentage / 100;
            size_t num_to_keep = original_count - num_to_remove;
            size_t start_index = num_to_remove;

            std::cout << "\nProcesando poda del " << percentage << "%...";
            std::string output_filename = base_name + "_pruned_opacity_" + std::to_string(percentage) + ".ply";
            std::filesystem::path output_path = output_dir / output_filename;
            write_pruned_ply(output_path.string(), all_gaussians, start_index, num_to_keep);
        }

    } catch (const std::exception& e) {
        std::cerr << "Ocurrió un error: " << e.what() << std::endl;
        return 1;
    }

    return 0;
}

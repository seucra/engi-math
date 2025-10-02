// compute_core/math_core.cpp

#include "math_core.h"
#include <Eigen/Dense>
#include <stdexcept>
#include <iostream>

void perform_matrix_vector_multiplication(
    const double* matrix_ptr,
    const double* vector_ptr,
    double* output_ptr,
    size_t dim
) {
    if (dim == 0 || !matrix_ptr || !vector_ptr || !output_ptr) {
        return; 
    }

    try {
        // Map the raw C++ pointers to Eigen types.
        const Eigen::Map<const Eigen::MatrixXd> M(matrix_ptr, dim, dim);
        const Eigen::Map<const Eigen::VectorXd> V_in(vector_ptr, dim);
        Eigen::Map<Eigen::VectorXd> V_out(output_ptr, dim);

        // Computation: V_out = M * V_in
        V_out = M * V_in;

    } catch (const std::exception& e) {
        std::cerr << "C++ Runtime Error during Eigen calculation: " << e.what() << std::endl;
    }
}

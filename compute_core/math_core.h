// compute_core/math_core.h

#pragma once

#include <cstddef> // for size_t

void perform_matrix_vector_multiplication(
    const double* matrix_ptr,
    const double* vector_ptr,
    double* output_ptr,
    size_t dim
);

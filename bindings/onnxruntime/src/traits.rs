/// Data type that can be used in tensors.
pub trait TensorDataType: Copy {
    /// Get the corresponding ONNX tensor element data type.
    fn get_onnx_tensor_element_data_type() -> onnxruntime_sys::ONNXTensorElementDataType;
}

impl TensorDataType for f32 {
    fn get_onnx_tensor_element_data_type() -> onnxruntime_sys::ONNXTensorElementDataType {
        onnxruntime_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_FLOAT
    }
}

impl TensorDataType for u8 {
    fn get_onnx_tensor_element_data_type() -> onnxruntime_sys::ONNXTensorElementDataType {
        onnxruntime_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT8
    }
}
impl TensorDataType for i8 {
    fn get_onnx_tensor_element_data_type() -> onnxruntime_sys::ONNXTensorElementDataType {
        onnxruntime_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_INT8
    }
}
impl TensorDataType for u16 {
    fn get_onnx_tensor_element_data_type() -> onnxruntime_sys::ONNXTensorElementDataType {
        onnxruntime_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT16
    }
}
impl TensorDataType for i16 {
    fn get_onnx_tensor_element_data_type() -> onnxruntime_sys::ONNXTensorElementDataType {
        onnxruntime_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_INT16
    }
}
impl TensorDataType for i32 {
    fn get_onnx_tensor_element_data_type() -> onnxruntime_sys::ONNXTensorElementDataType {
        onnxruntime_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_INT32
    }
}
impl TensorDataType for i64 {
    fn get_onnx_tensor_element_data_type() -> onnxruntime_sys::ONNXTensorElementDataType {
        onnxruntime_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_INT64
    }
}

impl TensorDataType for &str {
    fn get_onnx_tensor_element_data_type() -> onnxruntime_sys::ONNXTensorElementDataType {
        onnxruntime_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_STRING
    }
}

impl TensorDataType for bool {
    fn get_onnx_tensor_element_data_type() -> onnxruntime_sys::ONNXTensorElementDataType {
        onnxruntime_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_BOOL
    }
}

impl TensorDataType for f64 {
    fn get_onnx_tensor_element_data_type() -> onnxruntime_sys::ONNXTensorElementDataType {
        onnxruntime_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_DOUBLE
    }
}

impl TensorDataType for u32 {
    fn get_onnx_tensor_element_data_type() -> onnxruntime_sys::ONNXTensorElementDataType {
        onnxruntime_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT32
    }
}

impl TensorDataType for u64 {
    fn get_onnx_tensor_element_data_type() -> onnxruntime_sys::ONNXTensorElementDataType {
        onnxruntime_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT64
    }
}

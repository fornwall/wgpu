/*!
Reserved keywords for [WGSL][wgsl] (WebGPU Shading Language).

[wgsl]: https://gpuweb.github.io/gpuweb/wgsl.html
*/

// https://gpuweb.github.io/gpuweb/wgsl/#keyword-summary
/// Reserved keywords.
pub const RESERVED: &[&str] = &[
    // type-defining keywords
    "array",
    "atomic",
    "bool",
    "float32",
    "int32",
    "mat2x2",
    "mat2x3",
    "mat2x4",
    "mat3x2",
    "mat3x3",
    "mat3x4",
    "mat4x2",
    "mat4x3",
    "mat4x4",
    "pointer",
    "sampler",
    "sampler_comparison",
    "struct",
    "texture_1d",
    "texture_2d",
    "texture_2d_array",
    "texture_3d",
    "texture_cube",
    "texture_cube_array",
    "texture_multisampled_2d",
    "texture_storage_1d",
    "texture_storage_2d",
    "texture_storage_2d_array",
    "texture_storage_3d",
    "texture_depth_2d",
    "texture_depth_2d_array",
    "texture_depth_cube",
    "texture_depth_cube_array",
    "texture_depth_multisampled_2d",
    "uint32",
    "vec2",
    "vec3",
    "vec4",
    // other keywords
    "bitcast",
    "block",
    "break",
    "case",
    "continue",
    "continuing",
    "default",
    "discard",
    "else",
    "else_if",
    "enable",
    "fallthrough",
    "false",
    "fn",
    "for",
    "function",
    "if",
    "let",
    "loop",
    "private",
    "read",
    "read_write",
    "return",
    "storage",
    "switch",
    "true",
    "type",
    "uniform",
    "var",
    "workgroup",
    "write",
    // image format keywords
    "r8unorm",
    "r8snorm",
    "r8uint",
    "r8sint",
    "r16uint",
    "r16sint",
    "r16float",
    "rg8unorm",
    "rg8snorm",
    "rg8uint",
    "rg8sint",
    "r32uint",
    "r32sint",
    "r32float",
    "rg16uint",
    "rg16sint",
    "rg16float",
    "rgba8unorm",
    "rgba8unorm_srgb",
    "rgba8snorm",
    "rgba8uint",
    "rgba8sint",
    "bgra8unorm",
    "bgra8unorm_srgb",
    "rgb10a2unorm",
    "rg11b10float",
    "rg32uint",
    "rg32sint",
    "rg32float",
    "rgba16uint",
    "rgba16sint",
    "rgba16float",
    "rgba32uint",
    "rgba32sint",
    "rgba32float",
    // reserved keywords
    "asm",
    "bf16",
    "const",
    "do",
    "enum",
    "f16",
    "f64",
    "handle",
    "i8",
    "i16",
    "i64",
    "mat",
    "premerge",
    "regardless",
    "typedef",
    "u8",
    "u16",
    "u64",
    "unless",
    "using",
    "vec",
    "void",
    "while",
];

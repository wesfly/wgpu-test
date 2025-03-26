#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, 0.5, 0.0], color: [1.0, 0.0, 0.0] },  // 0
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] }, // 1
    Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },  // 2

    Vertex { position: [0.5, 0.5, 0.0], color: [1.0, 0.0, 1.0] },   // upper right
];

pub const INDICES: &[u16] = &[
    0, 1, 2,
    0, 2, 3,
];

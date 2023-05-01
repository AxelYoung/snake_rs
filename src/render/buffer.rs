use wgpu::util::DeviceExt;

use crate::systems::{GameState, GRID_SIZE, Cell};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2]
}

const SPRITE_COUNT: [u8; 2] = [3, 1];

const TILE_VERTS: [Vertex; 4] =  [
    Vertex { position: [1.0, 1.0, 0.0], tex_coords: [1.0, 0.0], }, // Top right
    Vertex { position: [-1.0, 1.0, 0.0], tex_coords: [0.0, 0.0], }, // Top left
    Vertex { position: [-1.0, -1.0, 0.0], tex_coords: [0.0, 1.0], }, // Bottom left
    Vertex { position: [1.0, -1.0, 0.0], tex_coords: [1.0, 1.0], }, // Bottom right
];

const TILE_INDIS: [u16; 6] = [
    0, 1, 2,
    0, 2, 3
];

impl Vertex {

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2
                }
            ]
        }
    }
}

pub fn create_buffers(device: &wgpu::Device, state: &GameState) -> (Option<wgpu::Buffer>, Option<wgpu::Buffer>, usize) {

    let mut verts : Vec<Vertex> = vec![];
    let mut indis : Vec<u16> = vec![];

    for (x, col) in state.board.iter().enumerate() {
        for (y, cell) in col.iter().enumerate() {
            if let Some(cell) = cell {
                create_tile(x, y, *cell, &mut verts, &mut indis)
            }
        } 
    }

    let vertex_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&verts),
            usage: wgpu::BufferUsages::VERTEX
        }
    );

    let index_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indis),
            usage: wgpu::BufferUsages::INDEX
        }
    );

    (Some(vertex_buffer), Some(index_buffer), indis.len())
}

fn create_tile(x: usize, y: usize, cell: Cell, verts: &mut Vec<Vertex>, indis: &mut Vec<u16>) {
    let mut tile_verts : Vec<Vertex> = TILE_VERTS.iter()
        .map(|v| Vertex {
            position: {
                [(v.position[0] + 1.0 - GRID_SIZE[0] as f32 + (x * 2) as f32) / GRID_SIZE[0] as f32, 
                (v.position[1] + 1.0 - GRID_SIZE[1] as f32 + (y * 2) as f32) / GRID_SIZE[1] as f32, 
                v.position[2]]
            },
            tex_coords: uv_from_index(v.tex_coords, match cell {
                Cell::Head => { [0, 0] },
                Cell::Tail => { [1, 0]},
                Cell::Apple => { [2, 0]}
            })
        })
        .collect();

    let mut tile_indis : Vec<u16> = TILE_INDIS.iter()
        .map(|i| i + verts.len() as u16)
        .collect();

    verts.append(&mut tile_verts);
    indis.append(&mut tile_indis);
}

fn uv_from_index(uv: [f32; 2], sprite_index: [u8; 2]) -> [f32; 2] {
    return [
        uv[0] / SPRITE_COUNT[0] as f32 + (sprite_index[0] as f32 / SPRITE_COUNT[0] as f32),
        uv[1] / SPRITE_COUNT[1] as f32 + (sprite_index[1] as f32 / SPRITE_COUNT[1] as f32),
    ]
}
//! Module containing renderer implementations.

use imgref::ImgVec;
use rgb::RGBA8;

use crate::{
    paint::GlyphTexture,
    Color,
    CompositeOperationState,
    ErrorKind,
    FillRule,
    ImageFilter,
    ImageId,
    ImageInfo,
    ImageSource,
    ImageStore,
};

mod opengl;
pub use opengl::OpenGl;

mod void;
pub use void::Void;

mod params;
pub use params::Params;

/// Represents drawable
#[derive(Copy, Clone, Default, Debug)]
pub struct Drawable {
    /// Fill vertices
    pub fill_verts: Option<(usize, usize)>,
    /// Stroke vertices
    pub stroke_verts: Option<(usize, usize)>,
}

/// Represents command type
#[derive(Debug)]
pub enum CommandType {
    /// Set render target
    SetRenderTarget(RenderTarget),
    /// Clear rectangle
    ClearRect {
        /// Rectangle x position
        x: u32,
        /// Rectangle y position
        y: u32,
        /// Rectangle width
        width: u32,
        /// Rectangle height
        height: u32,
        /// Clear color
        color: Color,
    },
    /// Convex fill
    ConvexFill {
        /// Fill parameters
        params: Params,
    },
    /// Concave fill
    ConcaveFill {
        /// Stencil parameters
        stencil_params: Params,
        /// Fill parameters
        fill_params: Params,
    },
    /// Stroke
    Stroke {
        /// Stroke parameters
        params: Params,
    },
    /// Stencil stroke
    StencilStroke {
        /// First parameters
        params1: Params,
        /// Second parameters
        params2: Params,
    },
    /// Triangles
    Triangles {
        /// Triangle parameters
        params: Params,
    },
    /// Render filtered image
    RenderFilteredImage {
        /// Target image id
        target_image: ImageId,
        /// Image filter
        filter: ImageFilter,
    },
}

/// Represents command
pub struct Command {
    /// Command type
    pub cmd_type: CommandType,
    /// Drawables
    pub drawables: Vec<Drawable>,
    /// Triangle vertices
    pub triangles_verts: Option<(usize, usize)>,
    /// Image
    pub image: Option<ImageId>,
    /// Glyph texture
    pub glyph_texture: GlyphTexture,
    /// Fill rule
    pub fill_rule: FillRule,
    /// Compsite operation
    pub composite_operation: CompositeOperationState,
}

impl Command {
    /// Create new Command with specified flavor
    pub fn new(flavor: CommandType) -> Self {
        Self {
            cmd_type: flavor,
            drawables: Default::default(),
            triangles_verts: Default::default(),
            image: Default::default(),
            glyph_texture: Default::default(),
            fill_rule: Default::default(),
            composite_operation: Default::default(),
        }
    }
}

/// Represents rendering target
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RenderTarget {
    /// Render to screen
    Screen,
    /// Render into image
    Image(ImageId),
}

/// This is the main renderer trait that the [Canvas](../struct.Canvas.html) draws to.
pub trait Renderer {
    /// Renderer image
    type Image;

    /// Set size
    fn set_size(&mut self, width: u32, height: u32, dpi: f32);

    /// Renderer image
    fn render(&mut self, images: &mut ImageStore<Self::Image>, verts: &[Vertex], commands: Vec<Command>);

    /// Alloc image
    fn alloc_image(&mut self, info: ImageInfo) -> Result<Self::Image, ErrorKind>;
    
    /// Update image
    fn update_image(&mut self, image: &mut Self::Image, data: ImageSource, x: usize, y: usize)
        -> Result<(), ErrorKind>;

    /// Delete image
    fn delete_image(&mut self, image: Self::Image, image_id: ImageId);

    /// Create screenchot
    fn screenshot(&mut self) -> Result<ImgVec<RGBA8>, ErrorKind>;
}

/// Vertex struct for specifying triangle geometry
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
#[repr(C)]
pub struct Vertex {
    /// Represents x 
    pub x: f32,
    /// Represents y 
    pub y: f32,
    /// Represents u 
    pub u: f32,
    /// Represents v 
    pub v: f32,
}

impl Vertex {
    /// Create new Vertex with specified params
    pub fn new(x: f32, y: f32, u: f32, v: f32) -> Self {
        Self { x, y, u, v }
    }

    /// Set params
    pub fn set(&mut self, x: f32, y: f32, u: f32, v: f32) {
        *self = Self { x, y, u, v };
    }
}

/// Represents Shader Type
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ShaderType {
    /// Represents Fill Gradient
    FillGradient,
    /// Represents Fill Image
    FillImage,
    /// Represents Stencil
    Stencil,
    /// Represents Fill Image Gradient
    FillImageGradient,
    /// Represents Filter Image
    FilterImage,
}

impl Default for ShaderType {
    fn default() -> Self {
        Self::FillGradient
    }
}

impl ShaderType {
    /// Represents shader type as f32
    pub fn to_f32(self) -> f32 {
        match self {
            Self::FillGradient => 0.0,
            Self::FillImage => 1.0,
            Self::Stencil => 2.0,
            Self::FillImageGradient => 3.0,
            Self::FilterImage => 4.0,
        }
    }
}

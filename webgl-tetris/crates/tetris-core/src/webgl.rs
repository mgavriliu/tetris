use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader};

// Tetromino colors [R, G, B]
const COLORS: [[f32; 3]; 8] = [
    [0.102, 0.102, 0.180], // 0: Empty (#1a1a2e)
    [0.000, 0.961, 1.000], // 1: I - Cyan
    [1.000, 0.843, 0.000], // 2: O - Yellow
    [0.616, 0.306, 0.871], // 3: T - Purple
    [0.000, 1.000, 0.498], // 4: S - Green
    [1.000, 0.420, 0.420], // 5: Z - Red
    [0.255, 0.412, 0.882], // 6: J - Blue
    [1.000, 0.549, 0.000], // 7: L - Orange
];

const VERTEX_SHADER: &str = r#"
    precision mediump float;

    attribute vec2 a_position;
    uniform vec2 u_offset;
    uniform vec2 u_resolution;
    uniform float u_cellSize;
    uniform vec2 u_gridOffset;
    uniform vec3 u_color;
    uniform float u_opacity;

    varying vec3 v_color;
    varying float v_opacity;
    varying vec2 v_localPos;

    void main() {
        vec2 cellPos = u_offset * u_cellSize + u_gridOffset;
        vec2 vertexPos = cellPos + a_position * (u_cellSize - 2.0) + 1.0;
        vec2 clipSpace = (vertexPos / u_resolution) * 2.0 - 1.0;
        gl_Position = vec4(clipSpace.x, -clipSpace.y, 0.0, 1.0);

        v_color = u_color;
        v_opacity = u_opacity;
        v_localPos = a_position;
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    precision mediump float;

    varying vec3 v_color;
    varying float v_opacity;
    varying vec2 v_localPos;

    uniform float u_cellSize;
    uniform float u_cornerRadius;

    float roundedBoxSDF(vec2 p, vec2 b, float r) {
        vec2 q = abs(p) - b + r;
        return min(max(q.x, q.y), 0.0) + length(max(q, 0.0)) - r;
    }

    void main() {
        float cellInner = u_cellSize - 2.0;
        vec2 p = v_localPos * cellInner - cellInner * 0.5;
        float d = roundedBoxSDF(p, vec2(cellInner * 0.5), u_cornerRadius);

        if (d > 0.5) {
            discard;
        }

        float alpha = 1.0 - smoothstep(-0.5, 0.5, d);
        gl_FragColor = vec4(v_color, v_opacity * alpha);
    }
"#;

const GRID_VERTEX_SHADER: &str = r#"
    precision mediump float;

    attribute vec2 a_position;
    uniform vec2 u_offset;
    uniform vec2 u_resolution;
    uniform float u_cellSize;
    uniform vec2 u_gridOffset;

    varying vec2 v_localPos;

    void main() {
        vec2 cellPos = u_offset * u_cellSize + u_gridOffset;
        vec2 vertexPos = cellPos + a_position * u_cellSize;
        vec2 clipSpace = (vertexPos / u_resolution) * 2.0 - 1.0;
        gl_Position = vec4(clipSpace.x, -clipSpace.y, 0.0, 1.0);
        v_localPos = a_position;
    }
"#;

const GRID_FRAGMENT_SHADER: &str = r#"
    precision mediump float;

    varying vec2 v_localPos;
    uniform float u_cellSize;

    void main() {
        vec3 fillColor = vec3(0.102, 0.102, 0.180);
        vec3 borderColor = vec3(0.165, 0.165, 0.290);

        float borderWidth = 1.0 / u_cellSize;
        float border = 0.0;

        if (v_localPos.x < borderWidth || v_localPos.x > 1.0 - borderWidth ||
            v_localPos.y < borderWidth || v_localPos.y > 1.0 - borderWidth) {
            border = 1.0;
        }

        vec3 color = mix(fillColor, borderColor, border);
        gl_FragColor = vec4(color, 1.0);
    }
"#;

pub struct WebGlRenderer {
    gl: WebGlRenderingContext,
    cell_program: WebGlProgram,
    grid_program: WebGlProgram,
    quad_buffer: WebGlBuffer,
    width: f32,
    height: f32,
    cell_size: f32,
    grid_width: u32,
    grid_height: u32,
    grid_offset: (f32, f32),
}

impl WebGlRenderer {
    pub fn new(
        canvas: &HtmlCanvasElement,
        grid_width: u32,
        grid_height: u32,
        cell_size: f32,
    ) -> Result<Self, JsValue> {
        let gl = canvas
            .get_context("webgl")?
            .ok_or("Failed to get WebGL context")?
            .dyn_into::<WebGlRenderingContext>()?;

        gl.enable(WebGlRenderingContext::BLEND);
        gl.blend_func(
            WebGlRenderingContext::SRC_ALPHA,
            WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
        );

        let cell_program = Self::create_program(&gl, VERTEX_SHADER, FRAGMENT_SHADER)?;
        let grid_program = Self::create_program(&gl, GRID_VERTEX_SHADER, GRID_FRAGMENT_SHADER)?;

        let quad_buffer = gl.create_buffer().ok_or("Failed to create buffer")?;
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&quad_buffer));

        let vertices: [f32; 12] = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0];

        unsafe {
            let array = js_sys::Float32Array::view(&vertices);
            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        let width = canvas.width() as f32;
        let height = canvas.height() as f32;

        Ok(Self {
            gl,
            cell_program,
            grid_program,
            quad_buffer,
            width,
            height,
            cell_size,
            grid_width,
            grid_height,
            grid_offset: (0.0, 0.0),
        })
    }

    fn compile_shader(
        gl: &WebGlRenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = gl
            .create_shader(shader_type)
            .ok_or("Failed to create shader")?;
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);

        if gl
            .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            let log = gl
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| "Unknown error".into());
            gl.delete_shader(Some(&shader));
            Err(format!("Shader compilation failed: {}", log))
        }
    }

    fn create_program(
        gl: &WebGlRenderingContext,
        vertex_src: &str,
        fragment_src: &str,
    ) -> Result<WebGlProgram, JsValue> {
        let vertex_shader =
            Self::compile_shader(gl, WebGlRenderingContext::VERTEX_SHADER, vertex_src)?;
        let fragment_shader =
            Self::compile_shader(gl, WebGlRenderingContext::FRAGMENT_SHADER, fragment_src)?;

        let program = gl.create_program().ok_or("Failed to create program")?;
        gl.attach_shader(&program, &vertex_shader);
        gl.attach_shader(&program, &fragment_shader);
        gl.link_program(&program);

        gl.delete_shader(Some(&vertex_shader));
        gl.delete_shader(Some(&fragment_shader));

        if gl
            .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            let log = gl
                .get_program_info_log(&program)
                .unwrap_or_else(|| "Unknown error".into());
            Err(JsValue::from_str(&format!(
                "Program linking failed: {}",
                log
            )))
        }
    }

    pub fn set_grid_offset(&mut self, x: f32, y: f32) {
        self.grid_offset = (x, y);
    }

    pub fn clear(&self) {
        self.gl.clear_color(0.039, 0.039, 0.102, 1.0);
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn render_grid(&self) {
        let gl = &self.gl;
        gl.use_program(Some(&self.grid_program));

        // Set uniforms
        let res_loc = gl.get_uniform_location(&self.grid_program, "u_resolution");
        gl.uniform2f(res_loc.as_ref(), self.width, self.height);

        let cell_size_loc = gl.get_uniform_location(&self.grid_program, "u_cellSize");
        gl.uniform1f(cell_size_loc.as_ref(), self.cell_size);

        let offset_loc = gl.get_uniform_location(&self.grid_program, "u_gridOffset");
        gl.uniform2f(offset_loc.as_ref(), self.grid_offset.0, self.grid_offset.1);

        // Bind quad buffer
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.quad_buffer));
        let pos_attrib = gl.get_attrib_location(&self.grid_program, "a_position") as u32;
        gl.enable_vertex_attrib_array(pos_attrib);
        gl.vertex_attrib_pointer_with_i32(pos_attrib, 2, WebGlRenderingContext::FLOAT, false, 0, 0);

        let cell_offset_loc = gl.get_uniform_location(&self.grid_program, "u_offset");

        // Draw each grid cell
        for y in 0..self.grid_height {
            for x in 0..self.grid_width {
                gl.uniform2f(cell_offset_loc.as_ref(), x as f32, y as f32);
                gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
            }
        }
    }

    pub fn render_cell(&self, x: u8, y: u8, color_index: u8, opacity: f32, corner_radius: f32) {
        let gl = &self.gl;
        gl.use_program(Some(&self.cell_program));

        // Set uniforms
        let res_loc = gl.get_uniform_location(&self.cell_program, "u_resolution");
        gl.uniform2f(res_loc.as_ref(), self.width, self.height);

        let cell_size_loc = gl.get_uniform_location(&self.cell_program, "u_cellSize");
        gl.uniform1f(cell_size_loc.as_ref(), self.cell_size);

        let grid_offset_loc = gl.get_uniform_location(&self.cell_program, "u_gridOffset");
        gl.uniform2f(
            grid_offset_loc.as_ref(),
            self.grid_offset.0,
            self.grid_offset.1,
        );

        let corner_loc = gl.get_uniform_location(&self.cell_program, "u_cornerRadius");
        gl.uniform1f(corner_loc.as_ref(), corner_radius);

        let offset_loc = gl.get_uniform_location(&self.cell_program, "u_offset");
        gl.uniform2f(offset_loc.as_ref(), x as f32, y as f32);

        let color = COLORS.get(color_index as usize).unwrap_or(&COLORS[0]);
        let color_loc = gl.get_uniform_location(&self.cell_program, "u_color");
        gl.uniform3f(color_loc.as_ref(), color[0], color[1], color[2]);

        let opacity_loc = gl.get_uniform_location(&self.cell_program, "u_opacity");
        gl.uniform1f(opacity_loc.as_ref(), opacity);

        // Bind quad buffer
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.quad_buffer));
        let pos_attrib = gl.get_attrib_location(&self.cell_program, "a_position") as u32;
        gl.enable_vertex_attrib_array(pos_attrib);
        gl.vertex_attrib_pointer_with_i32(pos_attrib, 2, WebGlRenderingContext::FLOAT, false, 0, 0);

        gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
    }

    pub fn render_cells(&self, cells: &[u8], corner_radius: f32) {
        for chunk in cells.chunks(4) {
            if chunk.len() == 4 {
                let x = chunk[0];
                let y = chunk[1];
                let color = chunk[2];
                let opacity = chunk[3] as f32 / 255.0;
                self.render_cell(x, y, color, opacity, corner_radius);
            }
        }
    }
}

// Preview renderer for smaller canvases (next piece, hold piece)
pub struct PreviewRenderer {
    gl: WebGlRenderingContext,
    cell_program: WebGlProgram,
    quad_buffer: WebGlBuffer,
    width: f32,
    height: f32,
    cell_size: f32,
}

impl PreviewRenderer {
    pub fn new(canvas: &HtmlCanvasElement, cell_size: f32) -> Result<Self, JsValue> {
        let gl = canvas
            .get_context("webgl")?
            .ok_or("Failed to get WebGL context")?
            .dyn_into::<WebGlRenderingContext>()?;

        gl.enable(WebGlRenderingContext::BLEND);
        gl.blend_func(
            WebGlRenderingContext::SRC_ALPHA,
            WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
        );

        let cell_program = WebGlRenderer::create_program(&gl, VERTEX_SHADER, FRAGMENT_SHADER)?;

        let quad_buffer = gl.create_buffer().ok_or("Failed to create buffer")?;
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&quad_buffer));

        let vertices: [f32; 12] = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0];

        unsafe {
            let array = js_sys::Float32Array::view(&vertices);
            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        let width = canvas.width() as f32;
        let height = canvas.height() as f32;

        Ok(Self {
            gl,
            cell_program,
            quad_buffer,
            width,
            height,
            cell_size,
        })
    }

    pub fn clear(&self) {
        self.gl.clear_color(0.039, 0.039, 0.102, 0.0);
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn render_cells(&self, cells: &[u8], corner_radius: f32) {
        self.clear();

        let gl = &self.gl;
        gl.use_program(Some(&self.cell_program));

        // Set common uniforms
        let res_loc = gl.get_uniform_location(&self.cell_program, "u_resolution");
        gl.uniform2f(res_loc.as_ref(), self.width, self.height);

        let cell_size_loc = gl.get_uniform_location(&self.cell_program, "u_cellSize");
        gl.uniform1f(cell_size_loc.as_ref(), self.cell_size);

        let grid_offset_loc = gl.get_uniform_location(&self.cell_program, "u_gridOffset");
        gl.uniform2f(grid_offset_loc.as_ref(), 0.0, 0.0);

        let corner_loc = gl.get_uniform_location(&self.cell_program, "u_cornerRadius");
        gl.uniform1f(corner_loc.as_ref(), corner_radius);

        // Bind quad buffer
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.quad_buffer));
        let pos_attrib = gl.get_attrib_location(&self.cell_program, "a_position") as u32;
        gl.enable_vertex_attrib_array(pos_attrib);
        gl.vertex_attrib_pointer_with_i32(pos_attrib, 2, WebGlRenderingContext::FLOAT, false, 0, 0);

        let offset_loc = gl.get_uniform_location(&self.cell_program, "u_offset");
        let color_loc = gl.get_uniform_location(&self.cell_program, "u_color");
        let opacity_loc = gl.get_uniform_location(&self.cell_program, "u_opacity");

        for chunk in cells.chunks(4) {
            if chunk.len() == 4 {
                let x = chunk[0];
                let y = chunk[1];
                let color_index = chunk[2];
                let opacity = chunk[3] as f32 / 255.0;

                gl.uniform2f(offset_loc.as_ref(), x as f32, y as f32);

                let color = COLORS.get(color_index as usize).unwrap_or(&COLORS[0]);
                gl.uniform3f(color_loc.as_ref(), color[0], color[1], color[2]);
                gl.uniform1f(opacity_loc.as_ref(), opacity);

                gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
            }
        }
    }
}

use winit::{self, event_loop::EventLoop, window::WindowBuilder};

use gl::{self};
use glutin::{
    self,
    config::{ColorBufferType, ConfigSurfaceTypes, ConfigTemplateBuilder, GlConfig},
    context::{GlProfile, NotCurrentGlContextSurfaceAccessor, PossiblyCurrentContext},
    display::{GetGlDisplay, GlDisplay},
    surface::{GlSurface, Surface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::GlWindow;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

use std::error::Error;
use std::ffi::CString;

/// Window used to create the main window for n3d.
pub struct Window {
    pub window: winit::window::Window,
    pub gl_surface: Surface<WindowSurface>,
    pub gl_context: PossiblyCurrentContext,
}

impl Window {
    pub fn new() -> Result<(Window, EventLoop<()>), Box<dyn Error>> {
        // create event loop and window
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_resizable(true)
            .with_title("N3D")
            .build(&event_loop)?;

        // create a display
        let display = unsafe {
            glutin::display::Display::new(
                window.raw_display_handle(),
                glutin::display::DisplayApiPreference::Egl,
            )?
        };

        // set the config template
        let template = ConfigTemplateBuilder::default()
            // .with_buffer_type(ColorBufferType::Rgb {
            //     r_size: 8,
            //     g_size: 8,
            //     b_size: 8,
            // })
            // .with_single_buffering(true)
            // .with_multisampling(4)
            // .with_surface_type(ConfigSurfaceTypes::all())
            // .prefer_hardware_accelerated(Some(true))
            .build();

        // find configs that match the template
        let gl_config = unsafe { display.find_configs(template) }?
            // return whichever has the most color depth
            .reduce(|config, acc| {
                match (
                    config.color_buffer_type(),
                    acc.color_buffer_type(),
                    config.num_samples(),
                    acc.num_samples(),
                ) {
                    (
                        Some(ColorBufferType::Rgb {
                            r_size: r1,
                            g_size: g1,
                            b_size: b1,
                        }),
                        Some(ColorBufferType::Rgb {
                            r_size: r2,
                            g_size: g2,
                            b_size: b2,
                        }),
                        c_samples,
                        a_samples,
                    ) => {
                        if r1 > r2 && g1 > g2 && b1 > b2 || c_samples > a_samples {
                            config
                        } else {
                            acc
                        }
                    }
                    (
                        Some(ColorBufferType::Rgb {
                            r_size: _,
                            g_size: _,
                            b_size: _,
                        }),
                        Some(_),
                        _,
                        _,
                    ) => config,
                    (_, _, _, _) => config,
                }
            })
            .expect("No available configs");
        println!("Config: samples {:?}", gl_config.num_samples());
        println!("Config: depth_size {:?}", gl_config.depth_size());
        println!(
            "Config: color buffer type {:?}",
            gl_config.color_buffer_type()
        );

        // build the surface attributes
        let attrs =
            window.build_surface_attributes(SurfaceAttributesBuilder::new().with_srgb(Some(true)));

        // create the surface on the display
        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)?
        };

        // Set the context attributes
        let att = glutin::context::ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .with_context_api(glutin::context::ContextApi::OpenGl(Some(
                glutin::context::Version { major: 4, minor: 6 },
            )))
            .build(Some(window.raw_window_handle()));

        // Make the context current
        let gl_context = unsafe {
            display
                .create_context(&gl_config, &att)?
                .make_current(&gl_surface)?
        };
        gl_surface.resize(
            &gl_context,
            std::num::NonZeroU32::new(1920).unwrap(),
            std::num::NonZeroU32::new(1080).unwrap(),
        );
        let _ = gl_surface.set_swap_interval(&gl_context, glutin::surface::SwapInterval::DontWait);

        gl::load_with(|s| display.get_proc_address(CString::new(s).unwrap().as_c_str()));

        Ok((
            Window {
                window,
                gl_surface,
                gl_context,
            },
            event_loop,
        ))
    }
}

use PrivateCapabilities;

use hal::{pass, image, pso, IndexType};
use hal::format::{Format, Swizzle, Properties, BufferFeature};
use hal::pso::{Comparison, StencilOp};
use metal::*;

impl PrivateCapabilities {
    pub fn map_format(&self, format: Format) -> Option<MTLPixelFormat> {
        use metal::MTLPixelFormat::*;
        use hal::format::Format as f;
        Some(match format {
            f::B5g6r5Unorm    if self.format_b5 => B5G6R5Unorm,
            f::B5g5r5a1Unorm  if self.format_b5 => BGR5A1Unorm,
            f::R8Srgb         if self.format_min_srgb_channels <= 1 => R8Unorm_sRGB,
            f::Rg8Srgb        if self.format_min_srgb_channels <= 2 => RG8Unorm_sRGB,
            f::Rgba8Srgb      if self.format_min_srgb_channels <= 4 => RGBA8Unorm_sRGB,
            f::Bgra8Srgb      if self.format_min_srgb_channels <= 4 => BGRA8Unorm_sRGB,
            f::D24UnormS8Uint if self.format_depth24_stencil8 => Depth24Unorm_Stencil8,
            f::D32FloatS8Uint if self.format_depth32_stencil8_filter || self.format_depth32_stencil8_none => Depth32Float_Stencil8,
            f::R8Unorm           => R8Unorm,
            f::R8Inorm           => R8Snorm,
            f::R8Uint            => R8Uint,
            f::R8Int             => R8Sint,
            f::Rg8Unorm          => RG8Unorm,
            f::Rg8Inorm          => RG8Snorm,
            f::Rg8Uint           => RG8Uint,
            f::Rg8Int            => RG8Sint,
            f::Rgba8Unorm        => RGBA8Unorm,
            f::Rgba8Inorm        => RGBA8Snorm,
            f::Rgba8Uint         => RGBA8Uint,
            f::Rgba8Int          => RGBA8Sint,
            f::Bgra8Unorm        => BGRA8Unorm,
            f::R16Unorm          => R16Unorm,
            f::R16Inorm          => R16Snorm,
            f::R16Uint           => R16Uint,
            f::R16Int            => R16Sint,
            f::R16Float          => R16Float,
            f::Rg16Unorm         => RG16Unorm,
            f::Rg16Inorm         => RG16Snorm,
            f::Rg16Uint          => RG16Uint,
            f::Rg16Int           => RG16Sint,
            f::Rg16Float         => RG16Float,
            f::Rgba16Unorm       => RGBA16Unorm,
            f::Rgba16Inorm       => RGBA16Snorm,
            f::Rgba16Uint        => RGBA16Uint,
            f::Rgba16Int         => RGBA16Sint,
            f::Rgba16Float       => RGBA16Float,
            f::A2r10g10b10Unorm  => RGB10A2Unorm,
            f::A2r10g10b10Uint   => RGB10A2Uint,
            f::B10g11r11Ufloat   => RG11B10Float,
            f::E5b9g9r9Ufloat    => RGB9E5Float,
            f::R32Uint           => R32Uint,
            f::R32Int            => R32Sint,
            f::R32Float          => R32Float,
            f::Rg32Uint          => RG32Uint,
            f::Rg32Int           => RG32Sint,
            f::Rg32Float         => RG32Float,
            f::Rgba32Uint        => RGBA32Uint,
            f::Rgba32Int         => RGBA32Sint,
            f::Rgba32Float       => RGBA32Float,
            f::D16Unorm          => Depth16Unorm,
            f::D32Float          => Depth32Float,
            f::Bc1RgbUnorm       if self.format_bc => BC1_RGBA,
            f::Bc1RgbSrgb        if self.format_bc => BC1_RGBA_sRGB,
            f::Bc2Unorm          if self.format_bc => BC2_RGBA,
            f::Bc2Srgb           if self.format_bc => BC2_RGBA_sRGB,
            f::Bc3Unorm          if self.format_bc => BC3_RGBA,
            f::Bc3Srgb           if self.format_bc => BC3_RGBA_sRGB,
            f::Bc4Unorm          if self.format_bc => BC4_RUnorm,
            f::Bc4Inorm          if self.format_bc => BC4_RSnorm,
            f::Bc5Unorm          if self.format_bc => BC5_RGUnorm,
            f::Bc5Inorm          if self.format_bc => BC5_RGSnorm,
            f::Bc6hUfloat        if self.format_bc => BC6H_RGBUfloat,
            f::Bc6hFloat         if self.format_bc => BC6H_RGBFloat,
            f::Bc7Unorm          if self.format_bc => BC7_RGBAUnorm,
            f::Bc7Srgb           if self.format_bc => BC7_RGBAUnorm_sRGB,
            f::EacR11Unorm       if self.format_eac_etc => EAC_R11Unorm,
            f::EacR11Inorm       if self.format_eac_etc => EAC_R11Snorm,
            f::EacR11g11Unorm    if self.format_eac_etc => EAC_RG11Unorm,
            f::EacR11g11Inorm    if self.format_eac_etc => EAC_RG11Snorm,
            f::Etc2R8g8b8Unorm   if self.format_eac_etc => ETC2_RGB8,
            f::Etc2R8g8b8Srgb    if self.format_eac_etc => ETC2_RGB8_sRGB,
            f::Etc2R8g8b8a1Unorm if self.format_eac_etc => ETC2_RGB8A1,
            f::Etc2R8g8b8a1Srgb  if self.format_eac_etc => ETC2_RGB8A1_sRGB,
            f::Astc4x4Unorm      if self.format_astc => ASTC_4x4_LDR, 
            f::Astc4x4Srgb       if self.format_astc => ASTC_4x4_sRGB, 
            f::Astc5x4Unorm      if self.format_astc => ASTC_5x4_LDR, 
            f::Astc5x4Srgb       if self.format_astc => ASTC_5x4_sRGB, 
            f::Astc5x5Unorm      if self.format_astc => ASTC_5x5_LDR, 
            f::Astc5x5Srgb       if self.format_astc => ASTC_5x5_sRGB, 
            f::Astc6x5Unorm      if self.format_astc => ASTC_6x5_LDR, 
            f::Astc6x5Srgb       if self.format_astc => ASTC_6x5_sRGB, 
            f::Astc6x6Unorm      if self.format_astc => ASTC_6x6_LDR, 
            f::Astc6x6Srgb       if self.format_astc => ASTC_6x6_sRGB, 
            f::Astc8x5Unorm      if self.format_astc => ASTC_8x5_LDR, 
            f::Astc8x5Srgb       if self.format_astc => ASTC_8x5_sRGB, 
            f::Astc8x6Unorm      if self.format_astc => ASTC_8x6_LDR, 
            f::Astc8x6Srgb       if self.format_astc => ASTC_8x6_sRGB, 
            f::Astc8x8Unorm      if self.format_astc => ASTC_8x8_LDR, 
            f::Astc8x8Srgb       if self.format_astc => ASTC_8x8_sRGB, 
            f::Astc10x5Unorm     if self.format_astc => ASTC_10x5_LDR, 
            f::Astc10x5Srgb      if self.format_astc => ASTC_10x5_sRGB, 
            f::Astc10x6Unorm     if self.format_astc => ASTC_10x6_LDR, 
            f::Astc10x6Srgb      if self.format_astc => ASTC_10x6_sRGB, 
            f::Astc10x8Unorm     if self.format_astc => ASTC_10x8_LDR, 
            f::Astc10x8Srgb      if self.format_astc => ASTC_10x8_sRGB, 
            f::Astc10x10Unorm    if self.format_astc => ASTC_10x10_LDR, 
            f::Astc10x10Srgb     if self.format_astc => ASTC_10x10_sRGB, 
            f::Astc12x10Unorm    if self.format_astc => ASTC_12x10_LDR, 
            f::Astc12x10Srgb     if self.format_astc => ASTC_12x10_sRGB, 
            f::Astc12x12Unorm    if self.format_astc => ASTC_12x12_LDR, 
            f::Astc12x12Srgb     if self.format_astc => ASTC_12x12_sRGB,
            // Not supported:
            // a8Unorm
            // agbr4Unorm
            // pvrtc_rgb_2bpp
            // pvrtc_rgb_2bpp_srgb
            // pvrtc_rgb_4bpp
            // pvrtc_rgb_4bpp_srgb
            // pvrtc_rgba_2bpp
            // pvrtc_rgba_2bpp_srgb
            // pvrtc_rgba_4bpp
            // pvrtc_rgba_4bpp_srgb
            // eac_rgba8
            // eac_rgba8_srgb
            // gbgr422
            // bgrg422
            // stencil8 (float-version)
            // x32_stencil8 (float-version)
            // x24_stencil8 (float-version)
            // bgra10_xr 
            // bgra10_xr_srgb 
            // bgr10_xr
            // bgr10_xr_srgb
            _ => return None,
        })
    }

    pub fn map_format_with_swizzle(&self, format: Format, swizzle: Swizzle) -> Option<MTLPixelFormat> {
        use hal::format::{Component::*, Format::*};
        use metal::MTLPixelFormat as Pf;
        match (format, swizzle) {
            (R8Unorm, Swizzle(Zero, Zero, Zero, R)) => Some(Pf::A8Unorm),
            (Rgba8Unorm, Swizzle(B, G, R, A)) => Some(Pf::BGRA8Unorm),
            (Bgra8Unorm, Swizzle(B, G, R, A)) => Some(Pf::RGBA8Unorm),
            (Bgra8Srgb, Swizzle(B, G, R, A)) => Some(Pf::RGBA8Unorm_sRGB),
            _ => {
                if swizzle != Swizzle::NO {
                    error!("Unsupported swizzle {:?} for format {:?}", swizzle, format);
                }
                self.map_format(format)
            }
        }
    }

    pub fn map_format_properties(&self, format: MTLPixelFormat) -> Properties {
        use hal::format::ImageFeature as If;
        use metal::MTLPixelFormat::*;

        let defaults = Properties {
            linear_tiling: If::empty(),
            optimal_tiling: If::SAMPLED | If::BLIT_SRC | If::BLIT_DST,
            buffer_features: BufferFeature::all(),
        };

        match format {
            A8Unorm => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            R8Unorm => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            R8Unorm_sRGB if self.format_r8unorm_srgb_all => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            R8Unorm_sRGB if self.format_r8unorm_srgb_no_write => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            R8Snorm if self.format_r8snorm_all => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            R8Uint => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::STORAGE 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            R8Sint => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::STORAGE 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            R16Unorm if self.format_r16_norm_all => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            R16Snorm if self.format_r16_norm_all => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            R16Uint => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::STORAGE 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            R16Sint => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::STORAGE 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            R16Float => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RG8Unorm => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RG8Unorm_sRGB if self.format_rg8unorm_srgb_all => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RG8Unorm_sRGB if self.format_rg8unorm_srgb_no_write => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RG8Snorm if self.format_rg8snorm_all => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RG8Uint => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            RG8Sint => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            B5G6R5Unorm if self.format_b5 => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            A1BGR5Unorm if self.format_b5 => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            ABGR4Unorm if self.format_b5 => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            BGR5A1Unorm if self.format_b5 => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            R32Uint if self.format_r32_all => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::STORAGE 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            R32Uint if self.format_r32_no_write => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            R32Sint if self.format_r32_all => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::STORAGE 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            R32Sint if self.format_r32_no_write => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            R32Float if self.format_r32float_no_write_no_filter => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::COLOR_ATTACHMENT 
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            R32Float if self.format_r32float_no_filter => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            R32Float if self.format_r32float_all => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RG16Unorm => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RG16Snorm => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RG16Float => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RGBA8Unorm => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RGBA8Unorm_sRGB if self.format_rgba8_srgb_no_write => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RGBA8Unorm_sRGB if self.format_rgba8_srgb_all => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RGBA8Snorm => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RGBA8Uint => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::STORAGE 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            RGBA8Sint => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::STORAGE 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            BGRA8Unorm => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            BGRA8Unorm_sRGB if self.format_rgba8_srgb_no_write => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            BGRA8Unorm_sRGB if self.format_rgba8_srgb_all => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RGB10A2Unorm if self.format_rgb10a2_unorm_all => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RGB10A2Unorm if self.format_rgb10a2_unorm_no_write => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RGB10A2Uint if self.format_rgb10a2_uint_color => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            RGB10A2Uint if self.format_rgb10a2_uint_color_write => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::STORAGE 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            RG11B10Float if self.format_rg11b10_all => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RG11B10Float if self.format_rg11b10_no_write => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RGB9E5Float if self.format_rgb9e5_all => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RGB9E5Float if self.format_rgb9e5_filter_only => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            RGB9E5Float if self.format_rgb9e5_no_write => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RG32Uint if self.format_rg32_color => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            RG32Sint if self.format_rg32_color => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            RG32Uint if self.format_rg32_color_write => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::COLOR_ATTACHMENT 
                    | If::STORAGE,
                ..defaults
            },
            RG32Sint if self.format_rg32_color_write => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::COLOR_ATTACHMENT 
                    | If::STORAGE,
                ..defaults
            },
            RG32Float if self.format_rg32float_all => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RG32Float if self.format_rg32float_color_blend => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::COLOR_ATTACHMENT 
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RG32Float if self.format_rg32float_no_filter => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::STORAGE 
                    | If::COLOR_ATTACHMENT 
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RGBA16Unorm => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RGBA16Snorm => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RGBA16Uint => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::STORAGE 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            RGBA16Sint => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::STORAGE 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            RGBA16Float => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RGBA32Uint if self.format_rgba32int_color => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            RGBA32Uint if self.format_rgba32int_color_write => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::COLOR_ATTACHMENT 
                    | If::STORAGE,
                ..defaults
            },
            RGBA32Sint if self.format_rgba32int_color => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            RGBA32Sint if self.format_rgba32int_color_write => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::COLOR_ATTACHMENT 
                    | If::STORAGE,
                ..defaults
            },
            RGBA32Float if self.format_rgba32float_all => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            RGBA32Float if self.format_rgba32float_color => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::COLOR_ATTACHMENT,
                ..defaults
            },
            RGBA32Float if self.format_rgba32float_color_write => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::COLOR_ATTACHMENT 
                    | If::STORAGE,
                ..defaults
            },
            EAC_R11Unorm if self.format_eac_etc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            EAC_R11Snorm if self.format_eac_etc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            EAC_RG11Unorm if self.format_eac_etc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            EAC_RG11Snorm if self.format_eac_etc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ETC2_RGB8 if self.format_eac_etc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ETC2_RGB8_sRGB if self.format_eac_etc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ETC2_RGB8A1 if self.format_eac_etc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ETC2_RGB8A1_sRGB if self.format_eac_etc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_4x4_LDR if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_4x4_sRGB if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_5x4_LDR if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_5x4_sRGB if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_5x5_LDR if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_5x5_sRGB if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_6x5_LDR if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_6x5_sRGB if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_6x6_LDR if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_6x6_sRGB if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_8x5_LDR if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_8x5_sRGB if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_8x6_LDR if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_8x6_sRGB if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_8x8_LDR if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_8x8_sRGB if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_10x5_LDR if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_10x5_sRGB if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_10x6_LDR if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_10x6_sRGB if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_10x8_LDR if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling     
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_10x8_sRGB if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_10x10_LDR if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_10x10_sRGB if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_12x10_LDR if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_12x10_sRGB if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_12x12_LDR if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            ASTC_12x12_sRGB if self.format_astc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            BC1_RGBA if self.format_bc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            BC1_RGBA_sRGB if self.format_bc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            BC2_RGBA if self.format_bc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            BC2_RGBA_sRGB if self.format_bc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            BC3_RGBA if self.format_bc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            BC3_RGBA_sRGB if self.format_bc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            BC4_RUnorm if self.format_bc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            BC4_RSnorm if self.format_bc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            BC5_RGUnorm if self.format_bc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            BC5_RGSnorm if self.format_bc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            BC6H_RGBUfloat if self.format_bc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            BC6H_RGBFloat if self.format_bc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            BC7_RGBAUnorm if self.format_bc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            BC7_RGBAUnorm_sRGB if self.format_bc => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            Depth16Unorm if self.format_depth16unorm => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            Depth32Float if self.format_depth32float_filter => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            Depth32Float if self.format_depth32float_none => Properties {
                optimal_tiling: If::empty(),
                ..defaults
            },
            Stencil8 => Properties {
                optimal_tiling: If::empty(),
                ..defaults
            },
            Depth24Unorm_Stencil8 if self.format_depth24_stencil8 => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            Depth32Float_Stencil8 if self.format_depth32_stencil8_filter => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR,
                ..defaults
            },
            Depth32Float_Stencil8 if self.format_depth32_stencil8_none => Properties {
                optimal_tiling: If::empty(),
                ..defaults
            },
            BGR10A2Unorm if self.format_bgr10a2_all => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::STORAGE
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            BGR10A2Unorm if self.format_bgr10a2_no_write => Properties {
                optimal_tiling: defaults.optimal_tiling 
                    | If::SAMPLED_LINEAR
                    | If::COLOR_ATTACHMENT
                    | If::COLOR_ATTACHMENT_BLEND,
                ..defaults
            },
            _ => defaults,
        }
    }
}

pub fn map_load_operation(operation: pass::AttachmentLoadOp) -> MTLLoadAction {
    use self::pass::AttachmentLoadOp::*;

    match operation {
        Load => MTLLoadAction::Load,
        Clear => MTLLoadAction::Clear,
        DontCare => MTLLoadAction::DontCare,
    }
}

pub fn map_store_operation(operation: pass::AttachmentStoreOp) -> MTLStoreAction {
    use self::pass::AttachmentStoreOp::*;

    match operation {
        Store => MTLStoreAction::Store,
        DontCare => MTLStoreAction::DontCare,
    }
}

pub fn map_write_mask(mask: pso::ColorMask) -> MTLColorWriteMask {
    let mut mtl_mask = MTLColorWriteMask::empty();

    if mask.contains(pso::ColorMask::RED) {
        mtl_mask |= MTLColorWriteMask::Red;
    }
    if mask.contains(pso::ColorMask::GREEN) {
        mtl_mask |= MTLColorWriteMask::Green;
    }
    if mask.contains(pso::ColorMask::BLUE) {
        mtl_mask |= MTLColorWriteMask::Blue;
    }
    if mask.contains(pso::ColorMask::ALPHA) {
        mtl_mask |= MTLColorWriteMask::Alpha;
    }

    mtl_mask
}

fn map_factor(factor: pso::Factor) -> MTLBlendFactor {
    use hal::pso::Factor::*;

    match factor {
        Zero => MTLBlendFactor::Zero,
        One => MTLBlendFactor::One,
        SrcColor => MTLBlendFactor::SourceColor,
        OneMinusSrcColor => MTLBlendFactor::OneMinusSourceColor,
        DstColor => MTLBlendFactor::DestinationColor,
        OneMinusDstColor => MTLBlendFactor::OneMinusDestinationColor,
        SrcAlpha => MTLBlendFactor::SourceAlpha,
        OneMinusSrcAlpha => MTLBlendFactor::OneMinusSourceAlpha,
        DstAlpha => MTLBlendFactor::DestinationAlpha,
        OneMinusDstAlpha => MTLBlendFactor::OneMinusDestinationAlpha,
        ConstColor => MTLBlendFactor::BlendColor,
        OneMinusConstColor => MTLBlendFactor::OneMinusBlendColor,
        ConstAlpha => MTLBlendFactor::BlendAlpha,
        OneMinusConstAlpha => MTLBlendFactor::OneMinusBlendAlpha,
        SrcAlphaSaturate => MTLBlendFactor::SourceAlphaSaturated,
        Src1Color => MTLBlendFactor::Source1Color,
        OneMinusSrc1Color => MTLBlendFactor::OneMinusSource1Color,
        Src1Alpha => MTLBlendFactor::Source1Alpha,
        OneMinusSrc1Alpha => MTLBlendFactor::OneMinusSource1Alpha,
    }
}

pub fn map_blend_op(operation: &pso::BlendOp) -> (MTLBlendOperation, MTLBlendFactor, MTLBlendFactor) {
    use hal::pso::BlendOp::*;

    match *operation {
        Add { src, dst } => (MTLBlendOperation::Add, map_factor(src), map_factor(dst)),
        Sub { src, dst } => (
            MTLBlendOperation::Subtract,
            map_factor(src),
            map_factor(dst),
        ),
        RevSub { src, dst } => (
            MTLBlendOperation::ReverseSubtract,
            map_factor(src),
            map_factor(dst),
        ),
        Min => (
            MTLBlendOperation::Min,
            MTLBlendFactor::Zero,
            MTLBlendFactor::Zero,
        ),
        Max => (
            MTLBlendOperation::Max,
            MTLBlendFactor::Zero,
            MTLBlendFactor::Zero,
        ),
    }
}

pub fn map_vertex_format(format: Format) -> Option<MTLVertexFormat> {
    use hal::format::Format as f;
    use metal::MTLVertexFormat::*;
    Some(match format {
        f::R8Unorm => UCharNormalized,
        f::R8Inorm => CharNormalized,
        f::R8Uint => UChar,
        f::R8Int => Char,
        f::Rg8Unorm => UChar2Normalized,
        f::Rg8Inorm => Char2Normalized,
        f::Rg8Uint => UChar2,
        f::Rg8Int => Char2,
        f::Rgb8Unorm => UChar3Normalized,
        f::Rgb8Inorm => Char3Normalized,
        f::Rgb8Uint => UChar3,
        f::Rgb8Int => Char3,
        f::Rgba8Unorm => UChar4Normalized,
        f::Rgba8Inorm => Char4Normalized,
        f::Rgba8Uint => UChar4,
        f::Rgba8Int => Char4,
        f::Bgra8Unorm => UChar4Normalized_BGRA,
        f::R16Unorm => UShortNormalized,
        f::R16Inorm => ShortNormalized,
        f::R16Uint => UShort,
        f::R16Int => Short,
        f::R16Float => Half,
        f::Rg16Unorm => UShort2Normalized,
        f::Rg16Inorm => Short2Normalized,
        f::Rg16Uint => UShort2,
        f::Rg16Int => Short2,
        f::Rg16Float => Half2,
        f::Rgb16Unorm => UShort3Normalized,
        f::Rgb16Inorm => Short3Normalized,
        f::Rgb16Uint => UShort3,
        f::Rgb16Int => Short3,
        f::Rgb16Float => Half3,
        f::Rgba16Unorm => UShort4Normalized,
        f::Rgba16Inorm => Short4Normalized,
        f::Rgba16Uint => UShort4,
        f::Rgba16Int => Short4,
        f::Rgba16Float => Half4,
        f::R32Uint => UInt,
        f::R32Int => Int,
        f::R32Float => Float,
        f::Rg32Uint => UInt2,
        f::Rg32Int => Int2,
        f::Rg32Float => Float2,
        f::Rgb32Uint => UInt3,
        f::Rgb32Int => Int3,
        f::Rgb32Float => Float3,
        f::Rgba32Uint => UInt4,
        f::Rgba32Int => Int4,
        f::Rgba32Float => Float4,
        _ => return None,
    })
}

pub fn resource_options_from_storage_and_cache(storage: MTLStorageMode, cache: MTLCPUCacheMode) -> MTLResourceOptions {
    MTLResourceOptions::from_bits(
        ((storage as u64) << MTLResourceStorageModeShift)
            | ((cache as u64) << MTLResourceCPUCacheModeShift),
    ).unwrap()
}

pub fn map_texture_usage(usage: image::Usage, tiling: image::Tiling) -> MTLTextureUsage {
    use hal::image::Usage as U;

    let mut texture_usage = MTLTextureUsage::PixelFormatView;
    if usage.intersects(U::COLOR_ATTACHMENT | U::DEPTH_STENCIL_ATTACHMENT) {
        texture_usage |= MTLTextureUsage::RenderTarget;
    }
    if usage.intersects(U::SAMPLED) {
        texture_usage |= MTLTextureUsage::ShaderRead;
    }
    if usage.intersects(U::STORAGE) {
        texture_usage |= MTLTextureUsage::ShaderRead | MTLTextureUsage::ShaderWrite;
    }

    match tiling {
        image::Tiling::Optimal => {
            // Note: for blitting, we do actual rendering, so we add more flags for TRANSFER_* usage
            if usage.contains(U::TRANSFER_DST) {
                texture_usage |= MTLTextureUsage::RenderTarget;
            }
            if usage.contains(U::TRANSFER_SRC) {
                texture_usage |= MTLTextureUsage::ShaderRead;
            }
        }
        image::Tiling::Linear => {}
    }

    texture_usage
}

pub fn map_texture_type(view_kind: image::ViewKind) -> MTLTextureType {
    use hal::image::ViewKind as Vk;
    match view_kind {
        Vk::D1 => MTLTextureType::D1,
        Vk::D1Array => MTLTextureType::D1Array,
        Vk::D2 => MTLTextureType::D2,
        Vk::D2Array => MTLTextureType::D2Array,
        Vk::D3 => MTLTextureType::D3,
        Vk::Cube => MTLTextureType::Cube,
        Vk::CubeArray => MTLTextureType::CubeArray,
    }
}

pub fn _map_index_type(index_type: IndexType) -> MTLIndexType {
    match index_type {
        IndexType::U16 => MTLIndexType::UInt16,
        IndexType::U32 => MTLIndexType::UInt32,
    }
}

pub fn map_compare_function(fun: Comparison) -> MTLCompareFunction {
    match fun {
        Comparison::Never => MTLCompareFunction::Never,
        Comparison::Less => MTLCompareFunction::Less,
        Comparison::LessEqual => MTLCompareFunction::LessEqual,
        Comparison::Equal => MTLCompareFunction::Equal,
        Comparison::GreaterEqual => MTLCompareFunction::GreaterEqual,
        Comparison::Greater => MTLCompareFunction::Greater,
        Comparison::NotEqual => MTLCompareFunction::NotEqual,
        Comparison::Always => MTLCompareFunction::Always,
    }
}

pub fn map_filter(filter: image::Filter) -> MTLSamplerMinMagFilter {
    match filter {
        image::Filter::Nearest => MTLSamplerMinMagFilter::Nearest,
        image::Filter::Linear => MTLSamplerMinMagFilter::Linear,
    }
}

pub fn map_wrap_mode(wrap: image::WrapMode) -> MTLSamplerAddressMode {
    match wrap {
        image::WrapMode::Tile => MTLSamplerAddressMode::Repeat,
        image::WrapMode::Mirror => MTLSamplerAddressMode::MirrorRepeat,
        image::WrapMode::Clamp => MTLSamplerAddressMode::ClampToEdge,
        image::WrapMode::Border => MTLSamplerAddressMode::ClampToBorderColor,
    }
}

pub fn map_extent(extent: image::Extent) -> MTLSize {
    MTLSize {
        width: extent.width as _,
        height: extent.height as _,
        depth: extent.depth as _,
    }
}

pub fn map_offset(offset: image::Offset) -> MTLOrigin {
    MTLOrigin {
        x: offset.x as _,
        y: offset.y as _,
        z: offset.z as _,
    }
}

pub fn map_stencil_op(op: StencilOp) -> MTLStencilOperation {
    match op {
        StencilOp::Keep => MTLStencilOperation::Keep,
        StencilOp::Zero => MTLStencilOperation::Zero,
        StencilOp::Replace => MTLStencilOperation::Replace,
        StencilOp::IncrementClamp => MTLStencilOperation::IncrementClamp,
        StencilOp::IncrementWrap => MTLStencilOperation::IncrementWrap,
        StencilOp::DecrementClamp => MTLStencilOperation::DecrementClamp,
        StencilOp::DecrementWrap => MTLStencilOperation::DecrementWrap,
        StencilOp::Invert => MTLStencilOperation::Invert,
    }
}

pub fn map_winding(face: pso::FrontFace) -> MTLWinding {
    match face {
        pso::FrontFace::Clockwise => MTLWinding::Clockwise,
        pso::FrontFace::CounterClockwise => MTLWinding::CounterClockwise,
    }
}

pub fn map_cull_face(face: pso::Face) -> Option<MTLCullMode> {
    match face {
        pso::Face::NONE => Some(MTLCullMode::None),
        pso::Face::FRONT => Some(MTLCullMode::Front),
        pso::Face::BACK => Some(MTLCullMode::Back),
        _ => None,
    }
}


import os
import sys
from PIL import Image

def generate_branding(input_path, output_dir):
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    try:
        # Open source logo (the white variant)
        img = Image.open(input_path).convert("RGBA")
        
        # Create a 512x512 black square
        size = 512
        square = Image.new("RGBA", (size, size), (0, 0, 0, 255))
        
        # Calculate resize to fit comfortably (80% width)
        target_w = int(size * 0.8)
        w_percent = (target_w / float(img.size[0]))
        target_h = int((float(img.size[1]) * float(w_percent)))
        
        img_resized = img.resize((target_w, target_h), Image.Resampling.LANCZOS)
        
        # Paste centered
        offset = ((size - target_w) // 2, (size - target_h) // 2)
        square.paste(img_resized, offset, img_resized)
        
        # Save App Logo
        app_logo_path = os.path.join(output_dir, "app_logo.png")
        square.save(app_logo_path, "PNG")
        print(f"Generated {app_logo_path}")
        
        # Save Favicon (32x32 PNG)
        favicon_png_path = os.path.join(output_dir, "favicon.png")
        square.resize((32, 32), Image.Resampling.LANCZOS).save(favicon_png_path, "PNG")
        print(f"Generated {favicon_png_path}")

        # Save Favicon (ICO)
        favicon_ico_path = os.path.join(output_dir, "favicon.ico")
        square.resize((32, 32), Image.Resampling.LANCZOS).save(favicon_ico_path, format="ICO")
        print(f"Generated {favicon_ico_path}")

        print("Branding asset generation complete.")

    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    # Use the logo from showcase assets as source
    logo_path = "/Users/vegarberentsen/Documents/PeakSuite/PeakUI/apps/showcase/assets/peak_logo_dark.png"
    # Save to showcase assets so Trunk can serve them
    output_path = "/Users/vegarberentsen/Documents/PeakSuite/PeakUI/apps/showcase/assets"
    generate_branding(logo_path, output_path)

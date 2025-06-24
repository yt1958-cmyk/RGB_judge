# coding: utf-8
"""select_image.py - Allow user to select rectangle on image and save result

Usage:
    python3 select_image.py input.jpg selection.json

The script displays the image, allows user to select a rectangular region with the
mouse. The resulting rectangle will be drawn in red and saved next to the image.
The coordinates of the rectangle are saved to selection.json.
"""
import cv2
import json
import sys


def main():
    if len(sys.argv) < 3:
        print("Usage: python3 select_image.py <input_image> <output_json>")
        return

    img_path = sys.argv[1]
    out_json = sys.argv[2]

    img = cv2.imread(img_path)
    if img is None:
        print(f"Failed to load {img_path}")
        return

    # Let user select ROI interactively
    r = cv2.selectROI("Select Region", img, False, False)
    x, y, w, h = r

    # Draw red rectangle
    img_rect = img.copy()
    cv2.rectangle(img_rect, (x, y), (x + w, y + h), (0, 0, 255), 2)
    cv2.imshow("Selected", img_rect)
    cv2.waitKey(0)

    # Save rectangle image and coordinates
    cv2.imwrite("selected_with_rect.png", img_rect)
    with open(out_json, 'w') as f:
        json.dump({'x': int(x), 'y': int(y), 'w': int(w), 'h': int(h)}, f)


if __name__ == "__main__":
    main()

import argparse
import open3d as o3d
# Create the parser
parser = argparse.ArgumentParser(description='convert_to_ascii & visualize')

# Add arguments
parser.add_argument("-v", action="store_true", help="visualize store_true")
parser.add_argument('--input', type=str, default="Dataset/kota_circuit2.ply", help='src path')
parser.add_argument('--output', type=str, default="temp/filtered_ply.ply", help='des path')
# Parse the command line arguments
args = parser.parse_args()

def visualize(path = "temp/filtered_ply.ply"):
    pcd = o3d.io.read_point_cloud(path)
    o3d.visualization.draw_geometries([pcd])

def convert_to_ascii(input_path = "Dataset/kota_circuit2.ply", output_path = "temp/ascii_ply.ply"):
    pcd = o3d.io.read_point_cloud(input_path)
    pcd.estimate_normals(search_param=o3d.geometry.KDTreeSearchParamHybrid(radius=0.1, max_nn=30))
    o3d.io.write_point_cloud(output_path, pcd, write_ascii=True)

if args.v:
    visualize(args.input)
else:
    convert_to_ascii(args.input, args.output)
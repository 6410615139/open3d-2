import open3d as o3d

path_kota_circuit2_ply = "Dataset/kota_circuit2.ply"
output_path = "temp/ascii_ply.ply"

pcd = o3d.io.read_point_cloud(path_kota_circuit2_ply)
pcd.estimate_normals(search_param=o3d.geometry.KDTreeSearchParamHybrid(radius=0.1, max_nn=30))
o3d.io.write_point_cloud(output_path, pcd, write_ascii=True)
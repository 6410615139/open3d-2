import open3d as o3d

path = "temp/filtered_ply.ply"
pcd = o3d.io.read_point_cloud(path)
o3d.visualization.draw_geometries([pcd])
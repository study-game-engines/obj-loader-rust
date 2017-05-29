extern crate tobj;

use std::path::Path;

#[test]
fn simple_triangle() {
    let m = tobj::load_obj(&Path::new("triangle.obj"));
    assert!(m.is_ok());
    let (models, mats) = m.unwrap();
    // We expect a single model with no materials
    assert_eq!(models.len(), 1);
    assert!(mats.is_empty());
    // Confirm our triangle is loaded correctly
    assert_eq!(models[0].name, "Triangle");
    let mesh = &models[0].mesh;
    assert!(mesh.normals.is_empty());
    assert!(mesh.texcoords.is_empty());
    assert_eq!(mesh.material_id, None);

    // Verify each position is loaded properly
    let expect_pos = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
    assert_eq!(mesh.positions, expect_pos);
    // Verify the indices are loaded properly
    let expect_idx = vec![0, 1, 2];
    assert_eq!(mesh.indices, expect_idx);
}

#[test]
fn multiple_face_formats() {
    let m = tobj::load_obj(&Path::new("quad.obj"));
    assert!(m.is_ok());
    let (models, mats) = m.unwrap();
    assert_eq!(models.len(), 3);
    assert!(mats.is_empty());

    // Confirm each object in the file was loaded properly
    assert_eq!(models[0].name, "Quad");
    let quad = &models[0].mesh;
    assert!(quad.normals.is_empty());
    assert_eq!(quad.material_id, None);
    let quad_expect_pos = vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0];
    let quad_expect_tex = vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0];
    let quad_expect_idx = vec![0, 1, 2, 0, 2, 3];
    assert_eq!(quad.positions, quad_expect_pos);
    assert_eq!(quad.texcoords, quad_expect_tex);
    assert_eq!(quad.indices, quad_expect_idx);

    assert_eq!(models[1].name, "Quad_face");
    let quad_face = &models[1].mesh;
    let quad_expect_normals = vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
    assert_eq!(quad_face.material_id, None);
    assert_eq!(quad_face.positions, quad_expect_pos);
    assert_eq!(quad_face.texcoords, quad_expect_tex);
    assert_eq!(quad_face.normals, quad_expect_normals);
    assert_eq!(quad_face.indices, quad_expect_idx);

    assert_eq!(models[2].name, "Tri_v_vn");
    let tri = &models[2].mesh;
    let tri_expect_pos = vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0];
    let tri_expect_normals = vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
    let tri_expect_idx = vec![0, 1, 2];
    assert_eq!(tri.material_id, None);
    assert_eq!(tri.positions, tri_expect_pos);
    assert_eq!(tri.normals, tri_expect_normals);
    assert_eq!(tri.indices, tri_expect_idx);
    assert!(tri.texcoords.is_empty());
}

#[test]
fn test_cornell() {
    let m = tobj::load_obj(&Path::new("cornell_box.obj"));
    assert!(m.is_ok());
    let (models, mats) = m.unwrap();
    assert_eq!(models.len(), 8);
    assert_eq!(mats.len(), 5);

    // Verify the floor loaded properly
    assert_eq!(models[0].name, "floor");
    let mesh = &models[0].mesh;
    assert_eq!(mesh.material_id, Some(0));
    let expect_indices = vec![0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11];
    // Will this be an issue with floating point precision?
    let expect_verts = vec![552.799988, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000,
                            0.000000, 0.000000, 559.200012, 549.599976, 0.000000, 559.200012,
                            290.000000, 0.000000, 114.000000, 240.000000, 0.000000, 272.000000,
                            82.000000, 0.000000, 225.000000, 130.000000, 0.000000, 65.000000,
                            472.000000, 0.000000, 406.000000, 314.000000, 0.000000, 456.000000,
                            265.000000, 0.000000, 296.000000, 423.000000, 0.000000, 247.000000];
    assert_eq!(mesh.indices, expect_indices);
    assert_eq!(mesh.positions, expect_verts);

    // Verify the light loaded properly
    assert_eq!(models[1].name, "light");
    let mesh = &models[1].mesh;
    assert_eq!(mesh.material_id, Some(3));
    let expect_indices = vec![0, 1, 2, 0, 2, 3];
    let expect_verts = vec![343.000000, 548.000000, 227.000000, 343.000000, 548.000000,
                            332.000000, 213.000000, 548.000000, 332.000000, 213.000000,
                            548.000000, 227.000000];
    assert_eq!(mesh.indices, expect_indices);
    assert_eq!(mesh.positions, expect_verts);

    // Verify the ceiling loaded properly
    assert_eq!(models[2].name, "ceiling");
    let mesh = &models[2].mesh;
    assert_eq!(mesh.material_id, Some(0));
    let expect_verts = vec![556.000000, 548.799988, 0.000000, 556.000000, 548.799988, 559.200012,
                            0.000000, 548.799988, 559.200012, 0.000000, 548.799988, 0.000000];
    assert_eq!(mesh.indices, expect_indices);
    assert_eq!(mesh.positions, expect_verts);

    // Verify the back wall loaded properly
    assert_eq!(models[3].name, "back_wall");
    let mesh = &models[3].mesh;
    assert_eq!(mesh.material_id, Some(0));
    let expect_verts = vec![549.599976, 0.000000, 559.200012, 0.000000, 0.000000, 559.200012,
                            0.000000, 548.799988, 559.200012, 556.000000, 548.799988, 559.200012];
    assert_eq!(mesh.indices, expect_indices);
    assert_eq!(mesh.positions, expect_verts);

    // Verify the green wall loaded properly
    assert_eq!(models[4].name, "green_wall");
    let mesh = &models[4].mesh;
    assert_eq!(mesh.material_id, Some(4));
    let expect_verts = vec![0.000000, 0.000000, 559.200012, 0.000000, 0.000000, 0.000000,
                            0.000000, 548.799988, 0.000000, 0.000000, 548.799988, 559.200012];
    assert_eq!(mesh.indices, expect_indices);
    assert_eq!(mesh.positions, expect_verts);

    // Verify the red wall loaded properly
    assert_eq!(models[5].name, "red_wall");
    let mesh = &models[5].mesh;
    assert_eq!(mesh.material_id, Some(1));
    let expect_verts = vec![552.799988, 0.000000, 0.000000, 549.599976, 0.000000, 559.200012,
                            556.000000, 548.799988, 559.200012, 556.000000, 548.799988, 0.000000];
    assert_eq!(mesh.indices, expect_indices);
    assert_eq!(mesh.positions, expect_verts);

    // Verify the short block loaded properly
    assert_eq!(models[6].name, "short_block");
    let mesh = &models[6].mesh;
    assert_eq!(mesh.material_id, Some(0));
    let expect_indices = vec![0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14,
                              12, 14, 15, 16, 17, 18, 16, 18, 19];
    let expect_verts =
        vec![130.000000, 165.000000, 65.000000, 82.000000, 165.000000, 225.000000, 240.000000,
             165.000000, 272.000000, 290.000000, 165.000000, 114.000000, 290.000000, 0.000000,
             114.000000, 290.000000, 165.000000, 114.000000, 240.000000, 165.000000, 272.000000,
             240.000000, 0.000000, 272.000000, 130.000000, 0.000000, 65.000000, 130.000000,
             165.000000, 65.000000, 290.000000, 165.000000, 114.000000, 290.000000, 0.000000,
             114.000000, 82.000000, 0.000000, 225.000000, 82.000000, 165.000000, 225.000000,
             130.000000, 165.000000, 65.000000, 130.000000, 0.000000, 65.000000, 240.000000,
             0.000000, 272.000000, 240.000000, 165.000000, 272.000000, 82.000000, 165.000000,
             225.000000, 82.000000, 0.000000, 225.000000];
    assert_eq!(mesh.indices, expect_indices);
    assert_eq!(mesh.positions, expect_verts);

    // Verify the tall block loaded properly
    assert_eq!(models[7].name, "tall_block");
    let mesh = &models[7].mesh;
    assert_eq!(mesh.material_id, Some(0));
    let expect_indices = vec![0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14,
                              12, 14, 15, 16, 17, 18, 16, 18, 19];
    let expect_verts =
        vec![423.000000, 330.000000, 247.000000, 265.000000, 330.000000, 296.000000, 314.000000,
             330.000000, 456.000000, 472.000000, 330.000000, 406.000000, 423.000000, 0.000000,
             247.000000, 423.000000, 330.000000, 247.000000, 472.000000, 330.000000, 406.000000,
             472.000000, 0.000000, 406.000000, 472.000000, 0.000000, 406.000000, 472.000000,
             330.000000, 406.000000, 314.000000, 330.000000, 456.000000, 314.000000, 0.000000,
             456.000000, 314.000000, 0.000000, 456.000000, 314.000000, 330.000000, 456.000000,
             265.000000, 330.000000, 296.000000, 265.000000, 0.000000, 296.000000, 265.000000,
             0.000000, 296.000000, 265.000000, 330.000000, 296.000000, 423.000000, 330.000000,
             247.000000, 423.000000, 0.000000, 247.000000];
    assert_eq!(mesh.indices, expect_indices);
    assert_eq!(mesh.positions, expect_verts);

    // Verify white material loaded properly
    assert_eq!(mats[0].name, "white");
    let mat = &mats[0];
    assert_eq!(mat.ambient, [0.0, 0.0, 0.0]);
    assert_eq!(mat.diffuse, [1.0, 1.0, 1.0]);
    assert_eq!(mat.specular, [0.0, 0.0, 0.0]);
    assert_eq!(mat.unknown_param.get("Ke").map(|s| s.as_ref()),
               Some("1 1 1"));
    assert_eq!(mat.illumination_model, None);

    // Verify red material loaded properly
    assert_eq!(mats[1].name, "red");
    let mat = &mats[1];
    assert_eq!(mat.ambient, [0.0, 0.0, 0.0]);
    assert_eq!(mat.diffuse, [1.0, 0.0, 0.0]);
    assert_eq!(mat.specular, [0.0, 0.0, 0.0]);
    assert_eq!(mat.illumination_model, Some(2));

    // Verify blue material loaded properly
    assert_eq!(mats[2].name, "blue");
    let mat = &mats[2];
    assert_eq!(mat.ambient, [0.0, 0.0, 0.0]);
    assert_eq!(mat.diffuse, [0.0, 0.0, 1.0]);
    assert_eq!(mat.specular, [0.0, 0.0, 0.0]);
    assert_eq!(mat.shininess, 10.0);
    assert_eq!(mat.unknown_param.len(), 1);
    assert_eq!(mat.unknown_param.get("crazy_unknown"),
               Some(&"Wierd stuff here".to_string()));

    // Verify light material loaded properly
    assert_eq!(mats[3].name, "light");
    let mat = &mats[3];
    assert_eq!(mat.ambient, [20.0, 20.0, 20.0]);
    assert_eq!(mat.diffuse, [1.0, 1.0, 1.0]);
    assert_eq!(mat.specular, [0.0, 0.0, 0.0]);
    assert_eq!(mat.dissolve, 0.8);
    assert_eq!(mat.optical_density, 1.25);

    // Verify green material loaded properly
    assert_eq!(mats[4].name, "green");
    let mat = &mats[4];
    assert_eq!(mat.ambient, [0.0, 0.0, 0.0]);
    assert_eq!(mat.diffuse, [0.0, 1.0, 0.0]);
    assert_eq!(mat.specular, [0.0, 0.0, 0.0]);
    assert_eq!(mat.ambient_texture, "dummy_texture.png");
    assert_eq!(mat.diffuse_texture, "dummy_texture.png");
    assert_eq!(mat.specular_texture, "dummy_texture.png");
    assert_eq!(mat.normal_texture, "dummy_texture.png");
    assert_eq!(mat.dissolve_texture, "dummy_texture.png");
}

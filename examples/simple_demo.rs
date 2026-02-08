use egui_gizmo::*;

fn main() {
    println!("Gizmo library compiled successfully!");
    println!("Check the main demo at: D:/Dev/Projetos/Eguizmo/demo/");
    
    // Simple test to ensure the library works
    let model_matrix = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    
    let view_matrix = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, -5.0, 1.0],
    ];
    
    let projection_matrix = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    
    let gizmo = Gizmo::new("test_gizmo")
        .view_matrix(view_matrix.into())
        .projection_matrix(projection_matrix.into())
        .model_matrix(model_matrix.into())
        .mode(GizmoMode::Rotate)
        .orientation(GizmoOrientation::Global);
    
    println!("Gizmo created successfully!");
}
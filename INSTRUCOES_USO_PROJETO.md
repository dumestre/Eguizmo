# Instruções para Usar egui-gizmo no Seu Projeto

Este documento fornece instruções específicas para integrar o `Eguizmo` no seu projeto Rust, especialmente se você estiver desenvolvendo uma engine de jogos.

## 1. Adicionando a Dependência

Para usar o `Eguizmo` no seu projeto, adicione a seguinte linha ao seu arquivo `Cargo.toml`:

```toml
[dependencies]
egui = "0.33.3"
egui-gizmo = { git = "https://github.com/dumestre/Eguizmo", branch = "main" }
glam = { version = "0.31.0", features = ["mint"] }
mint = "0.5.9"
```

## 2. Configuração Básica

Aqui está um exemplo mínimo de como configurar o gizmo no seu código:

```rust
use Eguizmo::*;

// Em algum lugar do seu código onde você gerencia objetos editáveis
struct EditableObject {
    transform: [[f32; 4]; 4],  // Matriz de transformação do objeto
    selected: bool,             // Se o objeto está selecionado
}

impl EditableObject {
    fn show_gizmo_if_selected(&mut self, ui: &mut egui::Ui, camera_view: &[[f32; 4]; 4], camera_proj: &[[f32; 4]; 4], gizmo_mode: GizmoMode) {
        if self.selected {
            let gizmo = Gizmo::new("obj_gizmo")
                .view_matrix((*camera_view).into())
                .projection_matrix((*camera_proj).into())
                .model_matrix(self.transform.into())
                .mode(gizmo_mode);

            if let Some(result) = gizmo.interact(ui) {
                self.transform = result.transform().into();
            }
        }
    }
}
```

## 3. Integração com Sua Engine

### Conversão de Matrizes

Se sua engine usa um sistema de matrizes diferente (por exemplo, `glam::Mat4`), você precisará converter entre os formatos:

```rust
use glam::Mat4;
use mint::ColumnMatrix4;

// Converter de sua engine para o formato do gizmo
fn to_gizmo_format(mat: &Mat4) -> ColumnMatrix4<f32> {
    mat.to_cols_array_2d().into()
}

// Converter do formato do gizmo para sua engine
fn from_gizmo_format(mint_mat: ColumnMatrix4<f32>) -> Mat4 {
    Mat4::from_cols_array_2d(&mint_mat.into())
}
```

### Exemplo de Integração Completa

```rust
use egui::{CentralPanel, Context};
use egui_gizmo::*;

struct GameEditor {
    selected_object_idx: Option<usize>,
    objects: Vec<EditableObject>,
    camera_view: [[f32; 4]; 4],
    camera_projection: [[f32; 4]; 4],
    gizmo_mode: GizmoMode,
}

impl GameEditor {
    fn render_editor(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            // Painel de seleção de objetos
            ui.collapsing("Objetos", |ui| {
                for (i, obj) in self.objects.iter_mut().enumerate() {
                    let selected = self.selected_object_idx.map_or(false, |idx| idx == i);
                    if ui.selectable_label(selected, format!("Objeto {}", i)).clicked() {
                        self.selected_object_idx = if selected { None } else { Some(i) };
                    }
                }
            });
            
            // Controles do gizmo
            ui.horizontal(|ui| {
                ui.label("Modo do Gizmo:");
                ui.radio_value(&mut self.gizmo_mode, GizmoMode::Rotate, "Rotacionar");
                ui.radio_value(&mut self.gizmo_mode, GizmoMode::Translate, "Mover");
                ui.radio_value(&mut self.gizmo_mode, GizmoMode::Scale, "Escalar");
            });
            
            // Área do gizmo - onde o objeto 3D seria renderizado
            ui.separator();
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                if let Some(idx) = self.selected_object_idx {
                    if let Some(obj) = self.objects.get_mut(idx) {
                        // Mostrar o gizmo para o objeto selecionado
                        obj.show_gizmo_if_selected(ui, &self.camera_view, &self.camera_projection, self.gizmo_mode);
                    }
                }
            });
        });
    }
}
```

## 4. Dicas Importantes

1. **Sistema de Coordenadas**: Certifique-se de que o sistema de coordenadas do seu motor de jogo seja compatível com o do gizmo.

2. **Matrizes de Câmera**: Assegure-se de que as matrizes de visualização e projeção da câmera estejam corretamente configuradas para que o gizmo funcione como esperado.

3. **Viewport**: O gizmo opera dentro do contexto do egui, então certifique-se de que ele esteja sendo renderizado na área correta da interface.

4. **Performance**: O gizmo é projetado para ser leve, mas se você tiver muitos objetos sendo editados simultaneamente, considere otimizações.

## 5. Exemplo Prático

Para ver um exemplo funcional, consulte o arquivo `DOCUMENTACAO_USO.md` que contém exemplos detalhados de integração.

## 6. Solução de Problemas

- Se o gizmo não aparecer: Verifique se as matrizes de câmera estão corretas e se o objeto está selecionado
- Se a interação não funcionar: Verifique se o gizmo está recebendo eventos de mouse corretamente
- Se as transformações forem incorretas: Verifique a conversão entre os formatos de matriz da sua engine e do gizmo

## 7. Contribuições

Este projeto está hospedado em [https://github.com/dumestre/Eguizmo](https://github.com/dumestre/Eguizmo). Sinta-se à vontade para contribuir com melhorias, correções de bugs ou novos recursos.
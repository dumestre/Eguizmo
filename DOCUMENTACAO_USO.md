# Documentação de Uso - Eguizmo

Esta documentação explica como integrar e utilizar a biblioteca `Eguizmo` no seu projeto Rust, especialmente em uma engine de jogos.

## Visão Geral

O `Eguizmo` é uma biblioteca que fornece gizmos 3D para manipulação de transformações (rotação, translação e escala) em aplicações que utilizam a biblioteca `egui`. Ele é particularmente útil para editores de cena, ferramentas de desenvolvimento de jogos e aplicações de modelagem 3D.

## Requisitos

- Rust 1.76 ou superior
- Biblioteca `egui` (versão 0.33.3 recomendada)
- Biblioteca `serde` (para serialização/deserialização)
- Matrizes 4x4 para representação de transformações (usando `mint` para interoperabilidade)

## Instalação

Para adicionar o `egui-gizmo` ao seu projeto, adicione a seguinte dependência ao seu `Cargo.toml`:

```toml
[dependencies]
egui = "0.33.3"
egui-gizmo = { git = "https://github.com/dumestre/Eguizmo", branch = "main" }
glam = { version = "0.31.0", features = ["mint"] }
mint = "0.5.9"
serde = { version = "1.0", features = ["derive"] }
```

## Integração Básica

### 1. Estrutura Básica

Aqui está um exemplo básico de como integrar o gizmo em sua aplicação egui:

```rust
use Eguizmo::*;

// Em sua estrutura de estado do jogo
struct GameState {
    model_matrix: [[f32; 4]; 4],  // Matriz de transformação do objeto
    view_matrix: [[f32; 4]; 4],   // Matriz de visualização da câmera
    projection_matrix: [[f32; 4]; 4], // Matriz de projeção
    gizmo_mode: GizmoMode,        // Modo atual do gizmo
    gizmo_orientation: GizmoOrientation, // Orientação do gizmo
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            // Matriz identidade como padrão
            model_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            // Matriz de visualização padrão (câmera atrás do objeto)
            view_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, -5.0, 1.0],
            ],
            // Matriz de projeção ortográfica padrão
            projection_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            gizmo_mode: GizmoMode::Rotate,
            gizmo_orientation: GizmoOrientation::Global,
        }
    }
}
```

### 2. Renderização do Gizmo

No loop de renderização da sua interface egui:

```rust
impl GameState {
    fn show_gizmo(&mut self, ui: &mut egui::Ui) {
        // Criar e interagir com o gizmo
        let gizmo = Gizmo::new("meu_gizmo")
            .view_matrix(self.view_matrix.into())           // Matriz de visualização da câmera
            .projection_matrix(self.projection_matrix.into()) // Matriz de projeção
            .model_matrix(self.model_matrix.into())         // Matriz do objeto a ser transformado
            .mode(self.gizmo_mode)                          // Modo atual (Rotate, Translate, Scale)
            .orientation(self.gizmo_orientation)            // Orientação (Global ou Local)
            .snapping(false);                               // Habilitar snapping (opcional)

        // Processar o resultado da interação
        if let Some(result) = gizmo.interact(ui) {
            // Atualizar a matriz do modelo com a transformação resultante
            self.model_matrix = result.transform().into();
        }
    }
}
```

## Modos de Operação

O gizmo suporta três modos principais:

1. **Rotação (`GizmoMode::Rotate`)**: Permite girar o objeto em torno dos eixos X, Y e Z
2. **Translação (`GizmoMode::Translate`)**: Permite mover o objeto ao longo dos eixos X, Y e Z
3. **Escala (`GizmoMode::Scale`)**: Permite dimensionar o objeto nos eixos X, Y e Z

## Orientações

O gizmo pode operar em duas orientações:

1. **Global (`GizmoOrientation::Global`)**: Os eixos de transformação são alinhados com o espaço global
2. **Local (`GizmoOrientation::Local`)**: Os eixos de transformação são alinhados com o espaço local do objeto

## Integração com Sua Engine de Jogos

### 1. Conversão de Matrizes

Se sua engine usa um formato diferente de matriz, você precisará converter entre os formatos. Aqui está um exemplo genérico:

```rust
// Supondo que sua engine use glam::Mat4
use glam::Mat4;

fn engine_to_gizmo_matrix(engine_matrix: Mat4) -> mint::ColumnMatrix4<f32> {
    engine_matrix.to_cols_array_2d().into()
}

fn gizmo_to_engine_matrix(gizmo_matrix: mint::ColumnMatrix4<f32>) -> Mat4 {
    Mat4::from_cols_array_2d(&gizmo_matrix.into())
}
```

### 2. Atualização de Objetos

Quando o gizmo retorna uma transformação, você pode aplicar isso ao seu objeto na engine:

```rust
impl GameState {
    fn apply_gizmo_transform(&mut self, object_id: usize) {
        // Converter a matriz do gizmo para o formato da sua engine
        let new_transform = gizmo_to_engine_matrix(self.model_matrix.into());
        
        // Aplicar a transformação ao objeto na sua engine
        // Exemplo hipotético - adapte para sua engine específica
        // self.engine.objects[object_id].set_transform(new_transform);
    }
}
```

## Personalização Visual

Você pode personalizar a aparência do gizmo usando a função `visuals`:

```rust
use egui::{Color32};

let custom_visuals = GizmoVisuals {
    x_color: Color32::from_rgb(255, 50, 0),    // Vermelho para eixo X
    y_color: Color32::from_rgb(50, 255, 0),    // Verde para eixo Y
    z_color: Color32::from_rgb(0, 50, 255),    // Azul para eixo Z
    s_color: Color32::from_rgb(255, 255, 255), // Branco para eixo da tela
    inactive_alpha: 0.5,                       // Transparência quando inativo
    highlight_alpha: 0.9,                      // Transparência quando destacado
    stroke_width: 4.0,                         // Espessura das linhas
    gizmo_size: 75.0,                          // Tamanho do gizmo em pixels
    ..Default::default()
};

let gizmo = Gizmo::new("meu_gizmo")
    .view_matrix(self.view_matrix.into())
    .projection_matrix(self.projection_matrix.into())
    .model_matrix(self.model_matrix.into())
    .mode(self.gizmo_mode)
    .visuals(custom_visuals);
```

## Dicas de Integração

1. **Viewport**: Certifique-se de que o viewport do gizmo corresponda à área onde você deseja exibi-lo
2. **Matrizes**: Assegure-se de que suas matrizes de visualização e projeção estejam corretamente configuradas
3. **Coordenadas**: Lembre-se que o sistema de coordenadas do egui é diferente do OpenGL/DirectX (Y positivo para baixo)
4. **Performance**: O gizmo é renderizado como parte da interface egui, então não afeta o pipeline de renderização 3D principal

## Exemplo Completo

Aqui está um exemplo completo de como integrar o gizmo em uma aplicação egui:

```rust
use egui::{CentralPanel, Context, Frame};
use egui_gizmo::*;

struct MyGameEditor {
    game_state: GameState,
}

impl MyGameEditor {
    fn new() -> Self {
        Self {
            game_state: GameState::default(),
        }
    }

    fn ui(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            // Painel de controle
            egui::SidePanel::left("controles").show_inside(ui, |ui| {
                ui.heading("Controles do Gizmo");
                
                ui.horizontal(|ui| {
                    ui.label("Modo:");
                    ui.radio_value(&mut self.game_state.gizmo_mode, GizmoMode::Rotate, "Rotacionar");
                    ui.radio_value(&mut self.game_state.gizmo_mode, GizmoMode::Translate, "Mover");
                    ui.radio_value(&mut self.game_state.gizmo_mode, GizmoMode::Scale, "Escalar");
                });
                
                ui.horizontal(|ui| {
                    ui.label("Orientação:");
                    ui.radio_value(&mut self.game_state.gizmo_orientation, GizmoOrientation::Global, "Global");
                    ui.radio_value(&mut self.game_state.gizmo_orientation, GizmoOrientation::Local, "Local");
                });
            });
            
            // Área do gizmo
            Frame::canvas(ui.style()).show(ui, |ui| {
                // Ajustar o tamanho disponível para o gizmo
                let desired_size = ui.available_size();
                ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());
                
                // Mostrar o gizmo
                self.game_state.show_gizmo(ui);
            });
        });
    }
}
```

## Solução de Problemas Comuns

1. **Gizmo não aparece**: Verifique se as matrizes de visualização/projeção estão corretas e se o viewport é adequado
2. **Interação não funciona**: Certifique-se de que o gizmo está recebendo eventos de mouse corretamente
3. **Transformações incorretas**: Verifique a ordem das operações e sistemas de coordenadas
4. **Desempenho ruim**: O gizmo é leve, mas se estiver em uma UI complexa, considere otimizações de repaint

## Conclusão

O `egui-gizmo` é uma ferramenta poderosa para adicionar manipuladores de transformação 3D à sua interface egui. Com a integração adequada, ele pode melhorar significativamente a experiência de edição em ferramentas de desenvolvimento de jogos e aplicações 3D.
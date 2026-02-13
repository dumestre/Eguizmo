# Eguizmo (Fork) - Guia Atualizado de Integracao

Este guia documenta como usar o `Eguizmo` (fork do `egui-gizmo`) no seu projeto, com foco no fluxo que voce ja usa no `Dengine`.

## O que e

`Eguizmo` fornece um gizmo 3D para manipulacao de transform (mover, rotacionar, escalar) dentro de UI `egui`.

## Instalacao

### Opcao 1: usar seu fork (recomendado)

No `Cargo.toml`:

```toml
[dependencies]
egui-gizmo = { git = "https://github.com/SEU_USUARIO/Eguizmo", branch = "main" }
glam = { version = "0.31.0", features = ["mint"] }
```

### Opcao 2: usar o repo original (referencia)

```toml
[dependencies]
egui-gizmo = { git = "https://github.com/dumestre/Eguizmo", branch = "main" }
glam = { version = "0.31.0", features = ["mint"] }
```

## Integracao basica

Imports:

```rust
use egui_gizmo::{Gizmo, GizmoMode, GizmoOrientation};
use glam::Mat4;
```

Exemplo minimo:

```rust
let gizmo = Gizmo::new("scene_transform_gizmo")
    .view_matrix(view.to_cols_array_2d().into())
    .projection_matrix(proj.to_cols_array_2d().into())
    .model_matrix(model_matrix.to_cols_array_2d().into())
    .mode(GizmoMode::Translate)
    .orientation(GizmoOrientation::Global)
    .viewport(viewport_rect);

if let Some(result) = gizmo.interact(ui) {
    model_matrix = Mat4::from(result.transform());
}
```

## Integracao no Dengine (estado atual)

No projeto atual:

- A viewport foi separada em `src/viewport.rs`.
- O gizmo e desenhado dentro de `ViewportPanel::show(...)`.
- O `main.rs` so instancia e chama a viewport.
- A area da viewport respeita:
  - largura dockada do `Inspector`
  - largura dockada do `Hierarchy`
  - altura reservada do `Project` no rodape

Arquivos relevantes:

- `src/viewport.rs`
- `src/main.rs`
- `src/hierarchy.rs` (getters de largura dockada)

## Controles de uso sugeridos

No editor, mantenha estados para:

- `GizmoMode`: `Translate`, `Rotate`, `Scale`
- `GizmoOrientation`: `Global`, `Local`
- Camera/projecao: perspectiva/ortografica

Exemplo de estado:

```rust
struct ViewportState {
    gizmo_mode: GizmoMode,
    gizmo_orientation: GizmoOrientation,
    model_matrix: Mat4,
}
```

## Conversao de matrizes

`Eguizmo` usa `mint` nos metodos publicos.
Com `glam` + feature `mint`, a conversao fica direta:

```rust
let mint_mat = mat4.to_cols_array_2d().into();
let mat4_back = Mat4::from(mint_mat);
```

## Erros comuns

1. Gizmo nao aparece:
- confirme que `viewport_rect` e valido (nao zero)
- confirme que view/projection estao corretas
- confirme que o gizmo esta sendo chamado dentro de uma area visivel do `egui`

2. Interacao do mouse nao funciona:
- evite sobrepor widgets clicaveis em cima do viewport
- confira se o `ui` usado no `gizmo.interact(ui)` e o da area correta

3. Transform "estranha":
- confira handness/sistema de coordenadas da camera
- teste primeiro com `Mat4::IDENTITY`

## Fluxo de update recomendado

1. calcular `viewport_rect`
2. montar `view` e `projection`
3. criar `Gizmo` com modo/orientacao atuais
4. aplicar `result.transform()` no objeto selecionado
5. renderizar cena usando a nova transform

## Versao documentada

- `egui`/`eframe`: `0.33.3`
- `egui-gizmo` (fork `Eguizmo`): branch `main`
- `glam`: `0.31` com feature `mint`

#[repr(usize)]
#[allow(dead_code)]
pub enum Keys {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
    I = 8,
    J = 9,
    K = 10,
    L = 11,
    M = 12,
    N = 13,
    O = 14,
    P = 15,
    Q = 16,
    R = 17,
    S = 18,
    T = 19,
    U = 20,
    V = 21,
    W = 22,
    X = 23,
    Y = 24,
    Z = 25,
    Num0 = 26,
    Num1 = 27,
    Num2 = 28,
    Num3 = 29,
    Num4 = 30,
    Num5 = 31,
    Num6 = 32,
    Num7 = 33,
    Num8 = 34,
    Num9 = 35,
    LeftShift = 36,
    RightShift = 37,
    LeftCtrl = 38,
    RightCtrl = 39,
    LeftAlt = 40,
    RightAlt = 41,
    LeftSuper = 42,
    RightSuper = 43,
    Space = 44,
    Plus = 45,
    Minus = 46,
}

#[repr(usize)]
pub enum Mouse {
    Left = 0,
    Right = 1,
    Middle = 2,
}

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
pub enum Menu {
    Config = 0,
    FPS = 1,
    Energy = 2,
    Camera = 3,
    Spawner = 4,
    Input = 5,
    Editor = 6,
    DragParams = 7,
    Advanced = 8,
    Debug = 9,
    Color = 10,
}

#[repr(usize)]
#[derive(Debug, PartialEq)]
#[derive(Clone)]
pub enum BodyType {
    RegularPolygon = 0,
    Rectangle = 1,
    Spring = 2,
    WeldJoint = 3,
    PivotJoint = 4,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ColorType {
    Random = 0,
    Set = 1,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InputMode {
    Spawn = 0,
    Edit = 1,
    Drag = 2,
    Move = 3,
    Nothing,
}

#[derive(Debug, PartialEq)]
pub enum DraggingState {
    NotDragging = 0,
    Dragging = 1,
    StartDragging = 2,
    StopDragging = 3,
}
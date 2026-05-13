#[derive(Debug, Clone, Default)]
pub(crate) struct Inset {
    pub(crate) top: i32, // en pixel
    pub(crate) right: i32,  // en pixel
    pub(crate) bottom: i32,  // en pixel
    pub(crate) left: i32,  // en pixel
}

impl Inset{
    pub fn new(top: i32, right: i32, bottom: i32, left: i32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn zero() -> Self {
        Self::new(0, 0, 0, 0)
    }

    pub fn all(&mut self, px: i32){
        self.top = px;
        self.right = px;
        self.bottom = px;
        self.left = px;
    }

    pub fn add_all(&mut self, px: i32){
        self.top += px;
        self.right += px;
        self.bottom += px;
        self.left += px;
    }

    pub fn horizontal(&mut self, px: i32){
        self.right = px;
        self.left = px;
    }
    
    pub fn vertical(&mut self, px: i32){
        self.top = px;
        self.bottom = px;
    }

    pub fn set(&mut self, top: i32, right: i32, bottom: i32, left: i32){
        self.top = top;
        self.right = right;
        self.bottom = bottom;
        self.left = left;
    }

    pub fn add(&mut self, top: i32, right: i32, bottom: i32, left: i32){
        self.top += top;
        self.right += right;
        self.bottom += bottom;
        self.left += left;
    }
}
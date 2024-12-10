use std::ops::{Add, Div, Mul, Sub, Neg};

// Define uma estrutura chamada Vec3 que representa um vetor 3D
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
  e: [f64; 3], // Um array de 3 elementos do tipo f64 (números de ponto flutuante de 64 bits)
}

pub type Point3 = Vec3; // Define um tipo Point3 como um vetor 3D

impl Vec3 {
  // Função para criar um novo vetor 3D
  pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
    Self { e: [e0, e1, e2] } // Inicializa o vetor com os valores fornecidos
  }

  // Função para obter o valor da coordenada x
  pub fn x(&self) -> f64 {
    self.e[0]
  }

  // Função para obter o valor da coordenada y
  pub fn y(&self) -> f64 {
    self.e[1]
  }

  // Função para obter o valor da coordenada z
  pub fn z(&self) -> f64 {
    self.e[2]
  }

  // Função para calcular o comprimento (magnitude) do vetor
  pub fn length(&self) -> f64 {
    self.length_squared().sqrt() // Raiz quadrada do comprimento ao quadrado
  }

  // Função para calcular o comprimento ao quadrado do vetor
  pub fn length_squared(&self) -> f64 {
    self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2] // Soma dos quadrados dos componentes
  }
}

// Implementação das operações aritméticas para a estrutura Vec3
impl Add for Vec3 {
  type Output = Self; // Define o tipo de retorno da operação de adição como Vec3

  // Implementação da operação de adição
  fn add(self, other: Self) -> Self::Output {
    Vec3::new(
      self.e[0] + other.e[0], // Soma das coordenadas x
      self.e[1] + other.e[1], // Soma das coordenadas y
      self.e[2] + other.e[2], // Soma das coordenadas z
    )
  }
}

impl Sub for Vec3 {
  type Output = Self; // Define o tipo de retorno da operação de subtração como Vec3

  // Implementação da operação de subtração
  fn sub(self, other: Self) -> Self::Output {
    Vec3::new(
      self.e[0] - other.e[0], // Subtração das coordenadas x
      self.e[1] - other.e[1], // Subtração das coordenadas y
      self.e[2] - other.e[2], // Subtração das coordenadas z
    )
  }
}

impl Mul<f64> for Vec3 {
  type Output = Self; // Define o tipo de retorno da operação de multiplicação como Vec3

  // Implementação da operação de multiplicação por um escalar
  fn mul(self, t: f64) -> Self::Output {
    Vec3::new(
      self.e[0] * t, // Multiplicação da coordenada x pelo escalar
      self.e[1] * t, // Multiplicação da coordenada y pelo escalar
      self.e[2] * t, // Multiplicação da coordenada z pelo escalar
    )
  }
}

impl Div<f64> for Vec3 {
  type Output = Self; // Define o tipo de retorno da operação de divisão como Vec3

  // Implementação da operação de divisão por um escalar
  fn div(self, t: f64) -> Self::Output {
    self * (1.0 / t) // Multiplicação pelo inverso do escalar
  }
}

// Implementação do operador unário `Neg` para a estrutura Vec3
impl Neg for Vec3 {
  type Output = Self; // Define o tipo de retorno da operação de negação como Vec3

  // Implementação da operação de negação
  fn neg(self) -> Self::Output {
      Vec3::new(
          -self.e[0], // Inverte a coordenada x
          -self.e[1], // Inverte a coordenada y
          -self.e[2], // Inverte a coordenada z
      )
  }
}

// Função para calcular o produto escalar (dot product) entre dois vetores
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
  u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2] // Soma dos produtos dos componentes correspondentes
}

// Função para calcular o produto vetorial (cross product) entre dois vetores
pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
  Vec3::new(
    u.e[1] * v.e[2] - u.e[2] * v.e[1], // Componente x do produto vetorial
    u.e[2] * v.e[0] - u.e[0] * v.e[2], // Componente y do produto vetorial
    u.e[0] * v.e[1] - u.e[1] * v.e[0], // Componente z do produto vetorial
  )
}

// Função para normalizar um vetor (torná-lo unitário)
pub fn unit_vector(v: &Vec3) -> Vec3 {
  *v / v.length() // Divisão do vetor pelo seu comprimento
}

# Generics

Generics is the topic of generalizing types and functionalities to broader cases.
This is extremely useful for reducing code duplication in many ways, but can call for some rather involved syntax.
Namely, being generic requires taking great care to specify over which types a generic type is actually considered valid.
The simplest and most common use of generics is for type parameters.

## Further information

- [Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Bounds](https://doc.rust-lang.org/rust-by-example/generics/bounds.html)



## Example about ZK
use std::fmt;
use std::marker::PhantomData; // 引入 PhantomData，这是 ZK 开发中极其常见的标记

// ============ 第1部分：基础 Trait 定义 ============

// Trait：定义哈希算法的行为接口
// Clone 是 Supertrait，表示实现 HashAlgorithm 的类型必须也能被 Clone
pub trait HashAlgorithm: Clone {
    fn hash(&self, data: &[u8]) -> Vec<u8>;
    fn name(&self) -> &str;
}

// SHA256 具体实现（单元结构体）
#[derive(Clone)]
pub struct SHA256;

impl HashAlgorithm for SHA256 {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        // 模拟哈希操作
        format!("SHA256({:?})", data).into_bytes()
    }
    fn name(&self) -> &str {
        "SHA256"
    }
}

// Blake3 具体实现
#[derive(Clone)]
pub struct Blake3;

impl HashAlgorithm for Blake3 {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        format!("Blake3({:?})", data).into_bytes()
    }
    fn name(&self) -> &str {
        "Blake3"
    }
}

// ============ 第2部分：泛型函数示例 ============

// T: PartialOrd 是 Trait Bound（特征约束）
// 只有实现了比较功能的类型才能传入此函数
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest { // 这里能使用 > 运算符是因为 T 实现了 PartialOrd
            largest = item;
        }
    }
    largest
}

// ============ 第3部分：单个泛型参数的结构体 ============

// 结构体泛型：T 代表任意类型
// x 和 y 必须是相同的类型 T
struct Point<T> {
    x: T,
    y: T,
}

// impl<T> 表示为所有可能的 Point<T> 实现方法
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

// ============ 第4部分：多泛型参数结构体 ============

// T 和 U 可以是不同类型，也可以是相同类型
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn new(x: T, y: U) -> Self {
        Point2 { x, y }
    }
}

// ============ 第5部分：特化（Specialization）实现 ============

// 这里的 impl 没有 <T>，而是直接写了 Point<f32>
// 这意味着 distance_from_origin 方法只存在于 Point<f32> 类型中
// Point<i32> 是无法调用此方法的
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// ============ 第6部分：综合例子 - 密码验证系统 ============

// 泛型结构体：H 被约束为必须实现 HashAlgorithm
pub struct PasswordHasher<H: HashAlgorithm> {
    algorithm: H,
    salt: String,
}

// 通用实现：适用于所有哈希算法
impl<H: HashAlgorithm> PasswordHasher<H> {
    pub fn new(algorithm: H, salt: String) -> Self {
        PasswordHasher { algorithm, salt }
    }
    
    pub fn hash_password(&self, password: &str) -> Vec<u8> {
        let combined = format!("{}{}", password, self.salt);
        self.algorithm.hash(combined.as_bytes())
    }
    
    pub fn verify(&self, password: &str, hash: &[u8]) -> bool {
        self.hash_password(password) == hash
    }
    
    pub fn get_algorithm(&self) -> &str {
        self.algorithm.name()
    }

    // 泛型方法：方法本身的泛型参数 NewH 独立于结构体的泛型参数 H
    // 允许我们在一个已有的泛型结构体中引入新的类型逻辑
    pub fn convert_to<NewH: HashAlgorithm>(
        self,
        new_algorithm: NewH,
    ) -> PasswordHasher<NewH> {
        PasswordHasher {
            algorithm: new_algorithm, // 替换算法
            salt: self.salt,          // 保留盐值
        }
    }
}

// 针对 SHA256 的特定优化实现
impl PasswordHasher<SHA256> {
    pub fn hash_batch(&self, passwords: &[&str]) -> Vec<Vec<u8>> {
        passwords.iter().map(|p| self.hash_password(p)).collect()
    }
}

// 针对 Blake3 的特定优化实现
impl PasswordHasher<Blake3> {
    pub fn hash_parallel(&self, data: &[u8]) -> Vec<u8> {
        println!("使用 Blake3 的并行特性");
        self.algorithm.hash(data)
    }
}

// ============ 第7部分：多泛型组合 ============

pub trait SaltGenerator {
    fn generate(&self) -> String;
}

#[derive(Clone)]
pub struct RandomSalt;

impl SaltGenerator for RandomSalt {
    fn generate(&self) -> String {
        "random_salt_12345".to_string()
    }
}

// 组合模式：同时依赖算法(H)和盐值生成器(S)
// 这是构建复杂系统（如区块链节点）时的常见模式
pub struct AdvancedHasher<H: HashAlgorithm, S: SaltGenerator> {
    algorithm: H,
    salt_generator: S,
}

impl<H: HashAlgorithm, S: SaltGenerator> AdvancedHasher<H, S> {
    pub fn new(algorithm: H, salt_gen: S) -> Self {
        AdvancedHasher {
            algorithm,
            salt_generator: salt_gen,
        }
    }
    pub fn hash(&self, password: &str) -> Vec<u8> {
        let salt = self.salt_generator.generate();
        let combined = format!("{}{}", password, salt);
        self.algorithm.hash(combined.as_bytes())
    }
}

// ============ 第9部分：ZK 零知识证明系统（重难点） ============

// 关联类型 (Associated Types)：type Field vs 泛型 <T>
// 在 Trait 中使用 type 定义占位符，实现者必须具体指定类型
pub trait Field: Clone + fmt::Debug + PartialEq {
    fn zero() -> Self;
    fn one() -> Self;
    fn add(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimpleField(pub u64);

const PRIME: u64 = 21888242871839275222246405745257275088548364400416034343698204186575808495617;

impl Field for SimpleField {
    fn zero() -> Self { SimpleField(0) }
    fn one() -> Self { SimpleField(1) }
    fn add(&self, other: &Self) -> Self {
        SimpleField((self.0 + other.0) % PRIME)
    }
    fn mul(&self, other: &Self) -> Self {
        // 注意：这里需要转化为 u128 防止乘法溢出，这是密码学运算的基本常识
        SimpleField(((self.0 as u128 * other.0 as u128) % PRIME as u128) as u64)
    }
}

// 椭圆曲线 Trait
pub trait Curve: Clone + fmt::Debug {
    // 这里使用了关联类型，强制 Field 和 Point 必须在此 Curve 定义下是兼容的
    type Field: Field;
    type Point: Clone + fmt::Debug;
    
    fn generator() -> Self::Point;
    fn scalar_mult(scalar: &Self::Field, point: &Self::Point) -> Self::Point;
}

#[derive(Clone, Debug)]
pub struct BN254; // 具体的曲线类型（通常是一个空结构体，仅用于标记类型）

#[derive(Clone, Debug)]
pub struct BN254Point(SimpleField);

impl Curve for BN254 {
    type Field = SimpleField; // 指定 BN254 使用 SimpleField
    type Point = BN254Point;
    
    fn generator() -> Self::Point {
        BN254Point(SimpleField(1))
    }
    fn scalar_mult(scalar: &Self::Field, _point: &Self::Point) -> Self::Point {
        // 简化的标量乘法模拟
        BN254Point(scalar.clone())
    }
}

// 电路定义 Trait
pub trait Circuit {
    type F: Field;
    // 返回值 Result<T, String> 本身也是泛型枚举
    fn evaluate(&self, inputs: &[Self::F]) -> std::result::Result<Vec<Self::F>, String>;
}

struct EqualityCircuit {
    value: SimpleField,
}

impl Circuit for EqualityCircuit {
    type F = SimpleField;
    
    fn evaluate(&self, inputs: &[SimpleField]) -> std::result::Result<Vec<SimpleField>, String> {
        if inputs[0] == self.value {
            Ok(vec![SimpleField::one()])
        } else {
            Err("值不相等".to_string())
        }
    }
}

// 承诺结构体
#[derive(Clone, Debug)]
pub struct Commitment<C: Curve> {
    value: C::Point, // 依赖于曲线 C 定义的点类型
}

// ZK 证明结构体
#[derive(Clone, Debug)]
pub struct ZKProof<C: Curve> {
    commitment: Commitment<C>,
    response: C::Field, // 依赖于曲线 C 定义的标量域类型
}

// 泛型证明器：核心逻辑
// PhantomData<C>: 编译器需要知道 struct 使用了泛型 C。
// 但如果 C 只在方法中使用而没有作为字段存储，编译器会报错。
// PhantomData 告诉编译器："逻辑上我拥有一个 C，尽管物理内存中没有它"。
pub struct Prover<C: Curve, Circ: Circuit<F = C::Field>> {
    curve: PhantomData<C>, 
    circuit: Circ,
    witness: Vec<C::Field>,
}

// 这里的 Circuit<F = C::Field> 约束非常关键
// 它强制电路使用的域(Field)必须和曲线使用的域一致，这是 ZK 安全性的基础
impl<C: Curve, Circ: Circuit<F = C::Field>> Prover<C, Circ> {
    pub fn new(circuit: Circ, witness: Vec<C::Field>) -> Self {
        Prover {
            curve: PhantomData, // 零大小，运行时不占用空间
            circuit,
            witness,
        }
    }
    
    pub fn commit(&self) -> Commitment<C> {
        let generator = C::generator();
        // 模拟 Pedersen Commitment
        let commitment_value = self.witness.iter().fold(C::Field::zero(), |acc, w| acc.add(w));
        let point = C::scalar_mult(&commitment_value, &generator);
        Commitment { value: point }
    }
    
    pub fn respond(&self, challenge: &C::Field) -> ZKProof<C> {
        let commitment = self.commit();
        let response = if !self.witness.is_empty() {
            challenge.mul(&self.witness[0])
        } else {
            C::Field::zero()
        };
        ZKProof {
            commitment,
            response,
        }
    }
}

// 泛型验证器
pub struct Verifier<C: Curve> {
    curve: PhantomData<C>,
}

impl<C: Curve> Verifier<C> {
    pub fn verify(proof: &ZKProof<C>, challenge: &C::Field) -> bool {
        // 模拟验证逻辑：g^r == C * g^c (此处仅为简单打印演示)
        println!("验证通过（简化演示）");
        true
    }
}

// ============ 第10部分：完整演示 ============

fn main() {
    println!("\n========== Rust 泛型完全指南 ==========\n");

    // ========== 泛型函数演示 ==========
    println!("【部分1：泛型函数】");
    let numbers = vec![34, 50, 25, 100, 65];
    println!("最大数: {}", largest(&numbers));

    // ========== 多泛型参数结构体 ==========
    println!("【部分3：多泛型参数结构体】");
    let p3 = Point2::new(5, 4.0);
    println!("混合类型 Point: ({}, {})\n", p3.x, p3.y);

    // ========== 密码验证系统 - SHA256 ==========
    println!("【部分5：SHA256 密码验证】");
    // 编译器此处通过类型推断知道 H 是 SHA256
    let sha_hasher = PasswordHasher::new(SHA256, "my_salt_123".to_string());
    let password = "my_secure_password";
    let hash1 = sha_hasher.hash_password(password);
    println!("哈希: {}", String::from_utf8_lossy(&hash1));
    println!("算法: {}\n", sha_hasher.get_algorithm());

    // ========== 算法转换 ==========
    println!("【部分9：转换算法（泛型方法）】");
    // 将 SHA256 Hasher 转换为 Blake3 Hasher
    let converted = sha_hasher.clone().convert_to(Blake3);
    println!("转换后算法: {}\n", converted.get_algorithm());

    // ========== ZK 零知识证明系统 ==========
    println!("【部分11：ZK 零知识证明系统】");
    
    // 1. 定义电路
    let circuit = EqualityCircuit {
        value: SimpleField(2),
    };
    // 2. 准备见证数据 (Witness)
    let witness = vec![SimpleField(2)];
    
    // 3. 实例化证明器
    // 注意：Rust 通常能推断类型，但这里我们显式指定 <BN254, _> 以展示泛型参数
    // _ 让编译器推断 Circuit 的具体类型
    let prover = Prover::<BN254, _>::new(circuit, witness);

    // 4. 生成承诺
    let commitment = prover.commit();
    println!("承诺: {:?}", commitment);

    // 5. 模拟挑战
    let challenge = SimpleField(3);
    println!("挑战: {:?}", challenge);

    // 6. 生成证明
    let proof = prover.respond(&challenge);
    println!("证明: {:?}", proof);

    // 7. 验证
    let _valid = Verifier::<BN254>::verify(&proof, &challenge);
    println!();

    // ========== 性能总结 ==========
    println!("========== 性能分析 ==========");
    println!("Rust 的泛型是单态化（Monomorphization）的：");
    println!("编译器会为每一个具体类型生成一份代码副本。");
    println!("例如 Prover<BN254> 和 Prover<BLS12_381> 会编译成两个完全不同的结构体。");
    println!("优点：运行时没有虚函数表开销，速度极快（C++ 模板级别性能）。");
    println!("缺点：二进制文件体积可能会增大。");
}
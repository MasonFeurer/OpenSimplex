use super::*;

#[test]
fn test() {
    let noise = Noise2d::new(627423, 1.0, 1.0);

    assert_eq!(noise.get(0.0, 0.0), 0.5);
    assert_eq!(noise.get(0.0, 0.0), noise.get(0.0, 0.0));

    assert_eq!(noise.get(4.0, 3.4), 0.5814482094778147);
    assert_eq!(noise.get(10.0, 2.432), 0.7353388997261322);
    assert_eq!(noise.get(10.0, 3.13), 0.6107336648252768);

    let noise = Noise2d::new(627423, 1.0, 4.0);

    assert_eq!(noise.get(0.0, 0.0), 2.0);
    assert_eq!(noise.get(0.0, 0.0), noise.get(0.0, 0.0));

    assert_eq!(noise.get(4.0, 3.4), 0.5814482094778147 * 4.0);
    assert_eq!(noise.get(10.0, 2.432), 0.7353388997261322 * 4.0);
    assert_eq!(noise.get(10.0, 3.13), 0.6107336648252768 * 4.0);

    let noise = Noise::new(0);
    let n2 = noise.clone();
    assert_eq!(noise.eval2d(4.0, 3.0), n2.eval2d(4.0, 3.0));
}

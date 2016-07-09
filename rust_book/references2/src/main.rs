fn main() {
    let (mut x, y) = (5, 6);

static z:i32 =2;
static q:i32 =3;
static w:i32 =2;

fn foo<'a>(mut x: &'a mut i32) {
//unsafe{
  *x= z;
  //}
}
   let m=&mut 3;
   println!("{:?}", m);
   foo(m);
   println!("{:?}", m);
   assert_eq!(&z,m);//what's the diff here between this
   assert_eq!(z,*m);//and this
   assert_eq!(&w,m);//this doesn't fail, because 2 == 2 but are they same ref though?!
   assert_eq!(&q,m);//this fails, obviously!
}


pragma circom 2.0.0;

include "helper.circom";

template ex_100() {
    signal input x1;
    signal input y1;
    signal input x2;
    signal input y2;
    signal input x3;
    signal input y3;

    component checker1 = exBabyCheck();
    component checker2 = exBabyCheck();
    component checker3 = exBabyCheck();
    
    checker1.x <== x1;
    checker1.y <== y1;
    checker2.x <== x2;
    checker2.y <== y2;
    checker3.x <== x3;
    checker3.y <== y3;

    component adder = exBabyAdd();
    adder.x1 <== x1;
    adder.y1 <== y1;
    adder.x2 <== x2;
    adder.y2 <== y2;
    adder.xout === x3;
    adder.yout === y3;
}

component main {public [x1,y1, x2, y2]} = ex_100();

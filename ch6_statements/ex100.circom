/*
    Copyright 2018 0KIMS association.

    This file is part of circom (Zero Knowledge Circuit Compiler).

    circom is a free software: you can redistribute it and/or modify it
    under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    circom is distributed in the hope that it will be useful, but WITHOUT
    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
    or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
    License for more details.

    You should have received a copy of the GNU General Public License
    along with circom. If not, see <https://www.gnu.org/licenses/>.
*/
pragma circom 2.0.0;

include "node_modules/circomlib/circuits/bitify.circom";
include "node_modules/circomlib/circuits/escalarmulfix.circom";

template exBabyCheck() {
    signal input x;
    signal input y;

    signal x2;
    signal y2;

    var a = 3;
    var d = 8;

    x2 <== x*x;
    y2 <== y*y;

    a*x2 + y2 === 1 + d*x2*y2;
}

template exBabyAdd() {
    signal input x1;
    signal input y1;
    signal input x2;
    signal input y2;
    signal output xout;
    signal output yout;

    signal beta;
    signal gamma;
    signal delta;
    signal tau;

    var a = 3;
    var d = 8;

    beta <== x1*y2;
    gamma <== y1*x2;
    delta <== (-a*x1+y1)*(x2 + y2);
    tau <== beta * gamma;

    xout <-- (beta + gamma) / (1+ d*tau);
    (1+ d*tau) * xout === (beta + gamma);

    yout <-- (delta + a*beta - gamma) / (1-d*tau);
    (1-d*tau)*yout === (delta + a*beta - gamma);
}

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

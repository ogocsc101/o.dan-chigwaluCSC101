fn main(){
    //amount
	let t:f64 = 450_000.00;
	let m:f64 = 1_500_000.00;
	let h:f64 = 750_000.00;
	let d:f64 = 2_850_000.00;
	let a:f64 = 250_000.00;

	//quantity 
	let q_t:f64 = 2.0;
	let q_m:f64 = 1.0;
	let q_h:f64 = 3.0;
	let q_d:f64 = 3.0;
	let q_a:f64 = 1.0;

	//sum
	let s = (t*q_t) + (m*q_m) + (h*q_h) + (d*q_d) + (a*q_a);
	println!("Sum is {}",s);

	//total quantity
	let tq = q_t + q_m + q_h + q_d +q_a;
	println!("Total Quantity is {}", tq);
	let a = s/tq;
	println!("Average is {}", a );
}
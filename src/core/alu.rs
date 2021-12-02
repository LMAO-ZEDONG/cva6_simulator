use crate::core::include::ariane_pkg::fu_data_t;
use crate::core::include::ariane_pkg::fu_op;
//adder lul (returns the result and alu_branch_res_o)
pub fn tick (fu_data_i: fu_data_t) -> (u64, bool) {
    let mut result: u64 = 0;

    if fu_data_i.operator == fu_op::ADD {
        result = add(&fu_data_i);
    }
    else if fu_data_i.operator == fu_op::SUB {
        result = sub(&fu_data_i);
    }
    (result, false)
}


// pub fn tick () -> (u64, bool) { //stub
//
//     (5, true)
// }

fn add(fu_data_i: &fu_data_t) -> u64 {
    fu_data_i.get_operand_a() + fu_data_i.get_operand_b()
}

fn sub(fu_data_i: &fu_data_t) -> u64 {
    fu_data_i.get_operand_a() - fu_data_i.get_operand_b()
}
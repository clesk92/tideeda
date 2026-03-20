// Top module that instantiates a sub-module
module hierarchy_top (
    input wire clk,
    input wire rst_n,
    output wire [7:0] data_out
);

    // Internal signal
    wire [7:0] internal_data;

    // Instantiate sub_module
    sub_module u_sub (
        .clk(clk),
        .rst_n(rst_n),
        .data_in(internal_data),
        .data_out(data_out)
    );

    // Some logic in top module
    assign internal_data = 8'hAB;

endmodule

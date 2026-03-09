module counter_with_load (
    input wire clk,
    input wire rst_n,
    input wire load_en,
    input wire [7:0] load_val,
    output reg [7:0] count
);

    wire [7:0] next_count;

    assign next_count = count + 1;

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            count <= 8'h00;
        end else if (load_en) begin
            count <= load_val;
        end else begin
            count <= next_count;
        end
    end

endmodule

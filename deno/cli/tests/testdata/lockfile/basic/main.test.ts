import { getValue } from "mod";

Deno.test("test", () => {
  const testing = 1 + getValue();
  if (testing !== 6) {
    throw "FAIL";
  }
});

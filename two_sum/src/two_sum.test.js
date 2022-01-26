"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const two_sum_1 = __importDefault(require("./two_sum"));
test("finds correct indicies in array", () => {
    const nums = [2, 7, 11, 15];
    const target = 9;
    expect((0, two_sum_1.default)(nums, target)).toEqual(expect.arrayContaining([0, 1]));
});

import { expect, test } from "vitest";
import { convertLength } from "../util/time";
import { useInterval } from "../util/poll";

test("covertLength", () => {
  const seconds = 180;
  const expected = "3:00";
  expect(convertLength(seconds)).toEqual(expected);
});

test("covertLength2", () => {
  const seconds = 232;
  const expected = "3:52";
  expect(convertLength(seconds)).toEqual(expected);
});

test("covertLengthLessThenMinute", () => {
  const seconds = 40;
  const expected = "0:40";
  expect(convertLength(seconds)).toEqual(expected);
});

test("covertLengthZero", () => {
  const seconds = 0;
  const expected = "0:00";
  expect(convertLength(seconds)).toEqual(expected);
});

test("poll", () => {
  const callback = () => { };
  const delay = 100;

  useInterval(callback, delay)

  expect(callback).toBeCalledTimes(1)
})


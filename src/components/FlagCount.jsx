import React from "react";

const FlagCount = ({ flagCount }) => {
  const lpad = (s, width, char) =>
    s.length >= width ? s : (new Array(width).join(char) + s).slice(-width);

  return (
    <div class="flex justify-center items-center grow h-14 text-2xl font-bold font-mono text-red-400 bg-gray-700 ">
      <div className="text-center">{lpad(`${flagCount}`, 3, "0")}</div>
    </div>
  );
};

export default FlagCount;

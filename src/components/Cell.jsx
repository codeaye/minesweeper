import React from "react";

const Cell = ({
  bg,
  text = "",
  colourCoded = false,
  isFlagMode = false,
  onClick,
  onClickFlagMode,
}) => {
  let colour = "";
  if (colourCoded) {
    switch (text) {
      case 1:
        colour = "text-blue-500";
        break;

      case 2:
        colour = "text-green-500";
        break;

      case 3:
        colour = "text-red-500";
        break;

      default:
        colour = "text-violet-500";
        break;
    }
  }

  return (
    <div
      className={`${bg} ${colour} border-gray-400 hover:bg-gray-200 border-b-2 border-l-2 hover:border-t-4 hover:border-l-4 m-0 w-8 h-8`}
      onClick={isFlagMode ? onClickFlagMode : onClick}
    >
      {text}
    </div>
  );
};

export default Cell;

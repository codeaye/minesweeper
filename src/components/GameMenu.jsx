import React from "react";
import FlagCount from "./FlagCount";
import ClearButton from "./ClearButton";

const GameMenu = ({
  flagCount,
  isFlagMode,
  setIsFlagMode,
  flagEmoji,
  otherEmoji,
  resetViewAndFlagCount,
}) => {
  return (
    <div class="w-full grid grid-cols-3 grid-rows-0 gap-0 h-min border-b-4 border-gray-900">
      <FlagCount flagCount={flagCount} />
      <div
        onClick={() => setIsFlagMode(!isFlagMode)}
        className="flex justify-center text-2xl items-center bg-gray-100 hover:bg-gray-300"
      >
        <div>{isFlagMode ? flagEmoji : otherEmoji}</div>
      </div>
      <ClearButton resetViewAndFlagCount={resetViewAndFlagCount} />
    </div>
  );
};

export default GameMenu;

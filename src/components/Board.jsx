import { invoke } from "@tauri-apps/api";
import React from "react";
import Cell from "./Cell";

const Board = ({ view, setView, isFlagMode, setFlagCount }) => {
  const open = async (x, y) => await setView(await invoke("open", { x, y }));
  const flag = async (x, y) => {
    await setView(await invoke("flag", { x, y }));
    await setFlagCount(await invoke("flag_count"));
  };

  return (
    <div className="h-80 w-full text-center align-middle grid grid-rows-10 grid-cols-10">
      {view.map((v) => {
        const { x, y } = v;
        switch (v.code) {
          case 0:
            return (
              <Cell
                isFlagMode={isFlagMode}
                key={`${x}-${y}`}
                bg="bg-gray-300"
                onClick={() => open(x, y)}
                onClickFlagMode={() => flag(x, y)}
              />
            );
          case 1:
            return (
              <Cell
                isFlagMode={isFlagMode}
                key={`${x}-${y}`}
                bg="bg-gray-100"
                onClick={() => open(x, y)}
                onClickFlagMode={() => flag(x, y)}
              />
            );
          case 2:
            return (
              <Cell
                isFlagMode={isFlagMode}
                key={`${x}-${y}`}
                bg="bg-gray-100"
                text="💣"
                onClick={() => open(x, y)}
                onClickFlagMode={() => flag(x, y)}
              />
            );
          case 3:
            return (
              <Cell
                isFlagMode={isFlagMode}
                key={`${x}-${y}`}
                bg="bg-gray-100"
                text="🚩"
                onClick={() => open(x, y)}
                onClickFlagMode={() => flag(x, y)}
              />
            );
          case 4:
            return (
              <Cell
                isFlagMode={isFlagMode}
                key={`${x}-${y}`}
                bg="bg-gray-100"
                text={v.count}
                colourCoded={true}
                onClick={() => open(x, y)}
                onClickFlagMode={() => flag(x, y)}
              />
            );
          default:
            break;
        }
      })}
    </div>
  );
};

export default Board;

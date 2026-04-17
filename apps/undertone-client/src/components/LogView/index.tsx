import React, { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";

type LogMessage = {
  message: string;
};
interface LogViewProps extends React.ComponentProps<"div"> {
  logs?: LogMessage[];
}

export const LogView = ({ ...props }: LogViewProps) => {
  return (
    <div {...props}>
      Im a log{" "}
      {logs &&
        logs.map((log, index) => {
          return (
            <div>
              <span key={index}>{log.message}</span>
            </div>
          );
        })}
    </div>
  );
};

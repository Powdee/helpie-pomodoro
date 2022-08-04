export interface IPomodoro {
  created?: Date;
  finished?: Date;
  resumed?: number[];
  paused?: number[];
  plans?: {
    repeat: number;
    time: number[][];
  }
}
export interface IData {
  x: number;
  y: number;
}

export interface IDatasetInfo {
  // name: string;
  data: IData[];
  csvX: string;
  csvY: string;
}

export interface IDataSchema {
  [key: string]: IDatasetInfo;
  // aX: IDatasetInfo;
  // aY: IDatasetInfo;
  // aZ: IDatasetInfo;
  // gX: IDatasetInfo,
  // gY: IDatasetInfo,
  // gZ: IDatasetInfo,
}

export interface IDataSPos {
  [key: string]: IDataSchema
}

interface ICycle {
  step: number[][]
  sel: [number, number]
}

export type IUpdator = (
data: IData[],
cycle: ICycle
) => void

export type INavUpdator = (
  updateLists: Function[],
  data: IData[],
  cycle: ICycle
) => void;

export interface IBoxResult  {
  [key: string]: number;
  min: number;
  max: number;
  median: number;
  q1: number;
  q3: number;
  IQR: number;
  lowerIQR: number;
  upperIQR: number,
}

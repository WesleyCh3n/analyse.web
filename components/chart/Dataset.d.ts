export interface IData {
  x: number;
  y: number;
}

export interface IDatasetInfo{
  name: string;
  mode: string;
  data: IData[];
  csvX: string;
  csvY: string;
}


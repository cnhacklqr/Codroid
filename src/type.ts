export enum Template {
  RustBinary,
  RustLibrary,
  Empty,
}

export interface ProjectInfos {
  infos: Array<ProjectInfo>;
}

export interface ProjectInfo {
  name: string;
  template: Template;
}

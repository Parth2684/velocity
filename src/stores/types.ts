



export enum Discovery {
  On = "on",
  Off = "off"
}

export enum CustomMatcherType {
  App = "app",
  Archive = "archive",
  Audio = "audio",
  Book = "book",
  Custom = "custom",
  Doc = "doc",
  Font = "font",
  Image = "image",
  Text = "text",
  Video = "video"
}

export interface AvailableDevice {
  ty_domain: string;
  sub_ty_domain: string | null;
  fullname: string;
  host: string;
  port: number;
  txt_properties: Map<string, string>
}

export interface Metadata {
  path: string;
  name: string;
  data_type: CustomMatcherType;
  file_size: number
}

export type StoreState = {
  availableDevices: Map<string, AvailableDevice>;
  files: Map<string, Metadata>
}

export type StoreAction = {
  serveAndConnectQuic: () => Promise<void>;
  scan: (discovery: Discovery) => Promise<void>;
  receiveCertAndConnectQuic: (txt_properties: Map<string, string>, otp: string) => Promise<void>;
  addAvailableDevice: (data: AvailableDevice) => void
  removeAvailableDevice: (name: string) => void
}
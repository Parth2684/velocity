import { create } from "zustand";
import { StoreAction, StoreState } from "./types";
import { invoke } from "@tauri-apps/api/core";

export const store = create<StoreState & StoreAction>((set, get) => ({
  availableDevices: new Map(),
  send_files: new Map(),
  receive_files: new Map(),
  otp: null,
  connectedTo: null,

  serveAndConnectQuic: async () => {
    try {
      await invoke("serve_and_connect_quic");
    } catch (err) {
      console.error("error serving: " + err);
    }
  },

  scan: async (discovery) => {
    try {
      await invoke("scan", {
        discovery,
      });
    } catch (err) {
      console.error("error scanning: " + err);
    }
  },
  receiveCertAndConnectQuic: async (txt_properties, otp) => {
    try {
      await invoke("receive_cert_and_connect_quic", {
        txt_properties,
        otp,
      });
    } catch (err) {
      console.error("error connecting device: " + err);
    }
  },
  addAvailableDevice: (data) => {
    set((state) => ({
      availableDevices: new Map(state.availableDevices).set(
        data.fullname,
        data,
      ),
    }));
  },
  removeAvailableDevice: (name) => {
    const beforeDelete = new Map(get().availableDevices);
    beforeDelete.delete(name);
    set({ availableDevices: beforeDelete });
  },
  setOtp: (otp) => {
    set({ otp });
  },
  addConnectedTo: (deviceName) => {
    set({ connectedTo: deviceName });
  },
  removeConnectedTo: () => {
    set({ connectedTo: null });
  },
  addSendTransfer: (transfers) => {
    const existingFiles = new Map(get().send_files);
    transfers.forEach((data, entry) => {
      existingFiles.set(entry, {
        path: data.path,
        name: data.name,
        data_type: data.data_type,
        file_size: data.file_size,
        transferred: 0,
        progress: 0,
        speed: 0
      });
    });
    set({ send_files: existingFiles });
  },
  
  addReceiveTransfer: (transfers) => {
    const existingFiles = new Map(get().send_files);
    transfers.forEach((data, entry) => {
      existingFiles.set(entry, {
        path: data.path,
        name: data.name,
        data_type: data.data_type,
        file_size: data.file_size,
        transferred: 0,
        progress: 0,
        speed: 0
      });
    });
    set({ send_files: existingFiles });
  }
}));

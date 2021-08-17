use std::ffi::c_void;
use std::ffi::CStr;
use std::path::PathBuf;

use imgui::{im_str, ImStr, ImString};

pub extern crate imgui_filedialog_sys as sys;

pub struct Context {
    ptr: *mut sys::ImGuiFileDialog,
}

unsafe fn ptr_to_string(ptr: *mut std::os::raw::c_char) -> String {
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}

#[must_use]
impl Context {
    fn new() -> Self {
        let igfd_ctx = unsafe { sys::IGFD_Create() };

        unsafe {
            // Set extension colours
            let col = sys::ImVec4 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
                w: 1.0,
            };
            sys::IGFD_SetExtentionInfos(
                igfd_ctx,
                im_str!(".txt").as_ptr(),
                col,
                im_str!("Exciting file!").as_ptr(),
            );
        };

        Self { ptr: igfd_ctx }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { sys::IGFD_Destroy(self.ptr) }
    }
}

pub struct FileDialog {
    id: ImString,
    context: Context,
    title: ImString,
    filters: ImString,
    min_size: [f32; 2],
    max_size: [f32; 2],
}

impl FileDialog {
    pub fn new(id: &ImStr) -> Self {
        Self {
            context: Context::new(),
            id: id.to_owned(),
            title: ImString::new("Choose a File"),
            filters: ImString::new(".*"),
            min_size: [200.0, 300.0],
            max_size: [700.0, 500.0],
        }
    }

    pub fn title(mut self, title: &ImStr) -> Self {
        self.title = title.to_owned();
        self
    }

    pub fn filters(mut self, filters: &ImStr) -> Self {
        self.filters = filters.to_owned();
        self
    }

    pub fn min_size(mut self, min_size: [f32; 2]) -> Self {
        self.min_size = min_size;
        self
    }

    pub fn max_size(mut self, max_size: [f32; 2]) -> Self {
        self.max_size = max_size;
        self
    }

    pub fn open_modal(&self) {
        unsafe {
            sys::IGFD_OpenPaneModal(
                self.context.ptr,
                self.id.as_ptr(),
                self.title.as_ptr(),
                self.filters.as_ptr(),
                im_str!(".").as_ptr(),
                im_str!("").as_ptr(),
                None,
                0.0,
                1,
                std::ptr::null_mut::<c_void>(),
                sys::ImGuiFileDialogFlags::from(0),
            );
        }
    }

    pub fn display(&self) -> bool {
        unsafe {
            sys::IGFD_DisplayDialog(
                self.context.ptr,
                self.id.as_ptr(),
                0,
                sys::ImVec2 {
                    x: self.min_size[0],
                    y: self.min_size[1],
                },
                sys::ImVec2 {
                    x: self.max_size[0],
                    y: self.max_size[1],
                },
            )
        }
    }

    pub fn close(&self) {
        unsafe {
            sys::IGFD_CloseDialog(self.context.ptr);
        }
    }

    pub fn is_ok(&self) -> bool {
        unsafe { sys::IGFD_IsOk(self.context.ptr) }
    }

    /// Path being browsed
    pub fn current_path(&self) -> Option<String> {
        if self.is_ok() {
            unsafe {
                let ptr = sys::IGFD_GetCurrentPath(self.context.ptr);
                Some(ptr_to_string(ptr))
            }
        } else {
            None
        }
    }

    /// Files selected in dialog by user
    pub fn selection(&self) -> Option<Selection> {
        Some(unsafe { Selection::new(sys::IGFD_GetSelection(self.context.ptr), &self.context) })
    }
}

pub struct Selection<'ui> {
    ptr: sys::IGFD_Selection,
    context: &'ui Context,
}

impl<'ui> Selection<'ui> {
    fn new(ptr: sys::IGFD_Selection, context: &'ui Context) -> Self {
        Selection { ptr, context }
    }
    pub fn files(&self) -> Vec<PathBuf> {
        let mut ret = vec![];
        for i in 0..self.ptr.count {
            let fixme = unsafe { ptr_to_string(sys::IGFD_GetCurrentPath(self.context.ptr)) };
            let fname = unsafe { ptr_to_string((*self.ptr.table.offset(i as isize)).fileName) };
            ret.push(std::path::PathBuf::new().join(fixme).join(fname));
        }
        ret
    }
}

impl Drop for Selection<'_> {
    fn drop(&mut self) {
        // TODO: Verify nothing needs dropped
    }
}

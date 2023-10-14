use std;
use winapi;
use sysinfo;
use std::io;
use std::io::Read;
use std::process::*;
use std::thread::sleep;
use std::{thread, time};
use winapi::shared::ntdef::FALSE;
use crate::get_process::get_proc;
use crate::offsets::player_offsets;
use winapi::um::winuser::MessageBoxA;
use winapi::ctypes::{c_void, __uint64};
use winapi::um::winuser::GetAsyncKeyState;
use sysinfo::{ProcessExt, System, SystemExt};
use crate::read_memory::{read_memory, write_memory};
use winapi::{um::winnt::PROCESS_ALL_ACCESS, shared::minwindef::HINSTANCE__};

mod get_process;
mod read_memory;
mod offsets;

fn main() {
  unsafe {
     
     let module_base = 0x140000000; 
     let entity_address: i64 = module_base + 0x1E92A8; 
     let process_id = get_proc("ac_client".trim().to_string());

     // Checks if the process is open
     if process_id == 0 {
        println!("-----------\n[*] Assault Cube is not open.\n-----------");
        return;

      } else { println!("------------\n[*] Complete. Assault Cube (rust_ac.exe) loaded.\n------------"); } 
      
   let handle = winapi::um::processthreadsapi::OpenProcess(PROCESS_ALL_ACCESS, FALSE.into(), process_id.try_into().unwrap());
   let mut entity: [u8; 4] = [0; 4];
   
      
   // get player address
   winapi::um::memoryapi::ReadProcessMemory(handle,
   entity_address as *const c_void,
   entity.as_mut_ptr().cast(),
   4,
   &mut 0);
   
   let player = u32::from_ne_bytes(entity);
   let player_state = player + player_offsets.get(&"player_state").unwrap();
   let player_name = player + player_offsets.get(&"username").unwrap();

   let health = player + player_offsets.get(&"health").unwrap();
   let armor = player + player_offsets.get(&"armor").unwrap();
   let grenades = player + player_offsets.get(&"grenades").unwrap();
   let recoil = player + player_offsets.get(&"recoil").unwrap();

   println!("[0x{:x}] is the player address", player);
   println!("[0x{:x}] is the player_state address", player_state);
   
   println!("[0x{:x}] is the health address", health);
      let inf_health = write_memory(handle, health as *mut c_void, 9999, false); //  Setting health to 999
       println!("Player health set to: {}", inf_health[0]);

   println!("[0x{:x}] is the armor address", armor);
      let inf_armor = write_memory(handle, armor as *mut c_void, 9999, false);
       println!("Player armor set to: {}", inf_armor[0]);

   println!("[0x{:x}] is the grenade address", grenades);
      let grenade_change1 = write_memory(handle, grenades as *const c_void, 128, false);
       println!("Player grenade amt set to: {}", grenade_change1[0]);

   println!("[0x{:x}] is the recoil address", recoil); 
   read_memory(handle, grenades as *const c_void, false)[0]; // Read the amount of grenades

   // Keybind loops
   println!("[*] Key-bind loops active.");
   let mut is_rapidfire_active = false;
   let mut is_recoil_active = false;
   let mut is_grenade_active = false;
   let mut max_stats = false;
   
      loop{
         const VK_DOWN: i32 = 0x28;
         const VK_SPACE: i32 = 0x20;
         const VK_DELETE: i32 = 0x2E;
         const VK_LEFT: i32 = 0x25;
         const VK_RIGHT: i32 = 0x27;
         const VK_SLASH: i32 = 0xDC;
         const VK_F1: i32 = 0x70;
         const VK_F2: i32 = 0x71;
         const VK_F3: i32 = 0x72;
         const VK_ZERO: i32 = 0x30;

         if GetAsyncKeyState(VK_LEFT) & 1 == 1 { // no recoil
            is_recoil_active = !is_recoil_active;
            
            if is_recoil_active == true {
               no_recoil(true);
               println!("[*] Activated no-recoil [<-]");

            } else if is_recoil_active == false {
               no_recoil(false);
               println!("[*] Disabled no-recoil. [<-]");

            }
         }

         if GetAsyncKeyState(VK_RIGHT) & 1 == 1 { // rapid-fire
            is_rapidfire_active = !is_rapidfire_active;

            if is_rapidfire_active == true {

               rapid_fire(true);
               println!("[*] Rapid-fire is now enabled. [->]");
            } else if is_rapidfire_active == false {

               rapid_fire(false);
               println!("[*] Rapid-fire is now disabled. [->]");
            }
         }

         if GetAsyncKeyState(VK_DELETE) & 1 == 1 { // end cheat process
            println!("[*] Closing cheats..[DEL]");
            exit(003);
            break;
         }

         if max_stats == true {
         write_memory(handle, health as *const c_void, 9999, false);
         write_memory(handle, armor as *const c_void, 9999, false);
         write_memory(handle, grenades as *const c_void, 9999, false);
         }
         if GetAsyncKeyState(VK_DOWN) & 1 == 1 { // give maximum health, grenades, and armor
            max_stats = !max_stats;
            println!("[*] Gave maximum stats. [↓]");

            
         }

         if GetAsyncKeyState(VK_F1) & 1 == 1 { // new username

            println!("[*] Enter your new username: [F1]");
            let mut user_input = String::new().trim().to_string();
            io::stdin()
               .read_line(&mut user_input)
               .expect("[*] Failed to read given username.");
            
            winapi::um::memoryapi::WriteProcessMemory(handle, 
            player_name as *mut c_void,
            user_input.as_mut_ptr() as *mut c_void,
            user_input.len(),
            &mut 0);
            println!("[*] Username set to: \"{}\"", user_input.trim());

         }

         if GetAsyncKeyState(VK_SLASH) & 1 == 1 { // no grenade throw cooldown
            is_grenade_active = !is_grenade_active;

            if is_grenade_active == true {
            let mut nop: [u8; 2] = [0x90, 0x90];
            
            winapi::um::memoryapi::WriteProcessMemory(handle,
               0x140102164 as *mut c_void,
               nop.as_mut_ptr().cast() as *mut c_void,
               2,
               &mut 0);
               
            println!("[*] No grenade cooldowns enabled. [\\]");
            } else {
               let mut original_instructions: [u8; 2] = [0x89, 0x08];

               winapi::um::memoryapi::WriteProcessMemory(handle,
                  0x140102164 as *mut c_void,
                  original_instructions.as_mut_ptr().cast() as *mut c_void,
                  2,
                  &mut 0);
               println!("[*] No grenade cooldowns disabled. [\\]");
            }
            
            
         }
         if GetAsyncKeyState(VK_ZERO) & 1 == 1 { // give current weapon max ammo

            let current_ammo = (module_base + 0x20d2e0) as *mut u32;

            let mut storage: [u8; 4] = [0; 4];

         winapi::um::memoryapi::ReadProcessMemory(handle,
            current_ammo as *mut c_void,
            storage.as_mut_ptr().cast(),
            storage.len(),
            &mut 0
         );
         let current_ammo_offset = u32::from_ne_bytes(storage) + 0x3b0;

         winapi::um::memoryapi::ReadProcessMemory(handle,
            current_ammo_offset as *mut c_void,
            storage.as_mut_ptr().cast(),
            storage.len(),
            &mut 0
         );
         let current_ammo_offset1 = u32::from_ne_bytes(storage) + 0x28;

         winapi::um::memoryapi::ReadProcessMemory(handle,
            current_ammo_offset1 as *mut c_void,
            storage.as_mut_ptr().cast(),
            storage.len(),
            &mut 0
         );

         let mut buffer: [u16; 1] = [9999];
         winapi::um::memoryapi::WriteProcessMemory(handle,
            u32::from_ne_bytes(storage) as *mut c_void,
            buffer.as_mut_ptr().cast(),
            storage.len(),
            &mut 0
         );

         println!("[*] Gave {} ammo to current held weapon. [0]", buffer[0]);
         }

         if get_proc("ac_client".trim().to_string()) == 0 {
            println!("[*] Assault Cube closed. Closing cheats.."); // Automatically close cheats if AC isn't running
            break;
            exit(003);
         }
      }

  }
  
}

fn no_recoil(is_enabled: bool){
unsafe {
   let process_id = get_proc("ac_client".trim().to_string());
   let handle = winapi::um::processthreadsapi::OpenProcess(PROCESS_ALL_ACCESS, FALSE.into(), process_id.try_into().unwrap());

 //  let mut recoil_memory_region = read_memory(handle, 0x1410345A as *mut c_void, false);
   let mut original_instructions: [u8; 5] = [0xF3, 0x0F, 0x11, 0x58, 0x44]; 

   if is_enabled == true {
      /* 
   if recoil_memory_region.contains(&0x90) {
      println!("------------\n[*] No recoil is already enabled.\n------------");
      return;
   }
      */
//println!("------------\n[*] No recoil enabled.\n------------");
   let mut buffer: [u64; 1] = [0x90];
   // NOP recoil instructions to give user no-recoil.
   for n in 0i64..4 {
      winapi::um::memoryapi::WriteProcessMemory(handle, 
         (0x14010345A + n) as *mut c_void, 
         buffer.as_mut_ptr().cast(), 
         buffer.len(),
         &mut 0);
      }
   }
   else {
 //     println!("------------\n[*] No recoil disabled.\n-------------");
      winapi::um::memoryapi::WriteProcessMemory(handle,
      0x14010345A as *mut c_void, 
      original_instructions.as_mut_ptr().cast(),
      original_instructions.len(),
      &mut 0);
     
  //   println!("[*] No-recoil disabled.");
   }
}
}

fn rapid_fire(is_enabled: bool){
   unsafe {
   let process_id = get_proc("ac_client".trim().to_string());
   let handle = winapi::um::processthreadsapi::OpenProcess(PROCESS_ALL_ACCESS, FALSE.into(), process_id.try_into().unwrap());

   let mut nopbuffer: [u64; 1] = [0x90];
   let mut original_bytecode: [u8; 6] = [0x89, 0xb5, 0x0c, 0x02, 0x00, 0x00]; 
   let mut rapidfire_memory: [u8; 100] = [0; 100];
   if(is_enabled == true){
   winapi::um::memoryapi::ReadProcessMemory(handle,
      0x14010614F as *const c_void,
      rapidfire_memory.as_mut_ptr().cast(),
      rapidfire_memory.len(),
      &mut 0);
    /* forgot to remove this 
      if rapidfire_memory.contains(&0x90) {
         println!("[*] Rapid-fire is already enabled!"); // In-case they try to enable rapid fire again
         return;
      }
    */
   for n in 0i64..6 {
      winapi::um::memoryapi::WriteProcessMemory(handle, 
         (0x14010614F + n) as *mut c_void, 
         nopbuffer.as_mut_ptr().cast(), 
         nopbuffer.len(),
         &mut 0);
      }
  // println!("[*] Rapid-fire enabled.");
   } else if is_enabled == false {

   // Rewrite original rapid-fire instructions after NOP
   winapi::um::memoryapi::WriteProcessMemory(handle, 
      0x14010614F as *mut c_void, 
      original_bytecode.as_mut_ptr().cast(), 
      original_bytecode.len(),
      &mut 0);
  // println!("[*] Rapid-fire disabled.");
   }
}
}






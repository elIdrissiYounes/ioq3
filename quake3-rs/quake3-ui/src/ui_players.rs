#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, libc)]
use bg_misc::bg_itemlist;
use bg_public_h::{
    animation_s, animation_t, gitem_s, gitem_t, itemType_t, unnamed_0, weapon_t, BOTH_DEAD1,
    BOTH_DEAD2, BOTH_DEAD3, BOTH_DEATH1, BOTH_DEATH2, BOTH_DEATH3, FLAG_RUN, FLAG_STAND,
    FLAG_STAND2RUN, IT_AMMO, IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP,
    IT_POWERUP, IT_TEAM, IT_WEAPON, LEGS_BACK, LEGS_BACKCR, LEGS_BACKWALK, LEGS_IDLE, LEGS_IDLECR,
    LEGS_JUMP, LEGS_JUMPB, LEGS_LAND, LEGS_LANDB, LEGS_RUN, LEGS_SWIM, LEGS_TURN, LEGS_WALK,
    LEGS_WALKCR, MAX_ANIMATIONS, MAX_TOTALANIMATIONS, TORSO_AFFIRMATIVE, TORSO_ATTACK,
    TORSO_ATTACK2, TORSO_DROP, TORSO_FOLLOWME, TORSO_GESTURE, TORSO_GETFLAG, TORSO_GUARDBASE,
    TORSO_NEGATIVE, TORSO_PATROL, TORSO_RAISE, TORSO_STAND, TORSO_STAND2, WP_BFG, WP_GAUNTLET,
    WP_GRAPPLING_HOOK, WP_GRENADE_LAUNCHER, WP_LIGHTNING, WP_MACHINEGUN, WP_NONE, WP_NUM_WEAPONS,
    WP_PLASMAGUN, WP_RAILGUN, WP_ROCKET_LAUNCHER, WP_SHOTGUN,
};
use q_math::{
    colorBlack, colorMdGrey, colorRed, colorWhite, colorYellow, g_color_table, vec3_origin,
    vectoangles, AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract,
    AnglesToAxis, AxisClear, MatrixMultiply, Q_fabs,
};
use q_shared_h::{
    byte, clipHandle_t, fileHandle_t, fsMode_t, orientation_t, qboolean, qfalse, qhandle_t, qtrue,
    sfxHandle_t, unnamed, va, vec3_t, vec4_t, vec_t, COM_Parse, COM_StripExtension, Com_Printf,
    Com_sprintf, Q_strcat, Q_stricmp, Q_strncpyz, CHAN_ANNOUNCER, CHAN_AUTO, CHAN_BODY, CHAN_ITEM,
    CHAN_LOCAL, CHAN_LOCAL_SOUND, CHAN_VOICE, CHAN_WEAPON, FS_APPEND, FS_APPEND_SYNC, FS_READ,
    FS_WRITE,
};
use stdlib::{atan2, atof, atoi, fabs, memset, rand, sin, strchr, tan};
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, refEntityType_t, refEntity_t, refdef_t,
    textureCompression_t, GLDRV_ICD, GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D, GLHW_GENERIC,
    GLHW_PERMEDIA2, GLHW_RAGEPRO, GLHW_RIVA128, RT_BEAM, RT_LIGHTNING, RT_MAX_REF_ENTITY_TYPE,
    RT_MODEL, RT_POLY, RT_PORTALSURFACE, RT_RAIL_CORE, RT_RAIL_RINGS, RT_SPRITE, TC_NONE, TC_S3TC,
    TC_S3TC_ARB,
};
use ui_addbots::{UI_AddBotsMenu, UI_AddBots_Cache};
use ui_atoms::{
    uis, UI_AdjustFrom640, UI_Argv, UI_ClampCvar, UI_ConsoleCommand, UI_CursorInRect,
    UI_Cvar_VariableString, UI_DrawBannerString, UI_DrawChar, UI_DrawHandlePic, UI_DrawNamedPic,
    UI_DrawProportionalString, UI_DrawProportionalString_AutoWrapped, UI_DrawRect, UI_DrawString,
    UI_FillRect, UI_ForceMenuOff, UI_Init, UI_IsFullscreen, UI_KeyEvent, UI_MouseEvent, UI_PopMenu,
    UI_ProportionalSizeScale, UI_ProportionalStringWidth, UI_PushMenu, UI_Refresh,
    UI_SetActiveMenu, UI_SetColor, UI_Shutdown,
};
use ui_cdkey::{UI_CDKeyMenu, UI_CDKeyMenu_Cache, UI_CDKeyMenu_f};
use ui_cinematics::{UI_CinematicsMenu, UI_CinematicsMenu_Cache, UI_CinematicsMenu_f};
use ui_confirm::{ConfirmMenu_Cache, UI_ConfirmMenu, UI_ConfirmMenu_Style, UI_Message};
use ui_connect::UI_DrawConnectScreen;
use ui_controls2::{Controls_Cache, UI_ControlsMenu};
use ui_credits::UI_CreditMenu;
use ui_demo2::{Demos_Cache, UI_DemosMenu};
use ui_display::{UI_DisplayOptionsMenu, UI_DisplayOptionsMenu_Cache};
use ui_gameinfo::{
    UI_CanShowTierVideo, UI_GetArenaInfoByMap, UI_GetArenaInfoByNumber, UI_GetAwardLevel,
    UI_GetBestScore, UI_GetBotInfoByName, UI_GetBotInfoByNumber, UI_GetCurrentGame,
    UI_GetNumArenas, UI_GetNumBots, UI_GetNumSPArenas, UI_GetNumSPTiers, UI_GetSpecialArenaInfo,
    UI_InitGameinfo, UI_LogAwardData, UI_NewGame, UI_SPUnlockMedals_f, UI_SPUnlock_f,
    UI_SetBestScore, UI_ShowTierVideo, UI_TierCompleted,
};
use ui_ingame::{InGame_Cache, UI_InGameMenu};
use ui_local_h::{
    _tag_menuframework, lerpFrame_t, menubitmap_s, menufield_s, menuframework_s, menulist_s,
    menutext_s, playerInfo_t, trap_CM_LerpTag, trap_Cvar_VariableValue, trap_Error,
    trap_FS_FCloseFile, trap_FS_FOpenFile, trap_FS_Read, trap_R_AddLightToScene,
    trap_R_AddRefEntityToScene, trap_R_ClearScene, trap_R_RegisterModel,
    trap_R_RegisterShaderNoMip, trap_R_RegisterSkin, trap_R_RenderScene, trap_S_StartLocalSound,
    uiStatic_t,
};
use ui_main::{
    ui_browserGameType, ui_browserMaster, ui_browserShowEmpty, ui_browserShowFull,
    ui_browserSortKey, ui_cdkeychecked, UI_RegisterCvars, UI_UpdateCvars,
};
use ui_menu::{MainMenu_Cache, UI_MainMenu};
use ui_mfield::{MField_Draw, MenuField_Draw, MenuField_Init, MenuField_Key};
use ui_mods::{UI_ModsMenu, UI_ModsMenu_Cache};
use ui_network::{UI_NetworkOptionsMenu, UI_NetworkOptionsMenu_Cache};
use ui_playermodel::{PlayerModel_Cache, UI_PlayerModelMenu};
use ui_playersettings::{PlayerSettings_Cache, UI_PlayerSettingsMenu};
use ui_preferences::{Preferences_Cache, UI_PreferencesMenu};
use ui_qmenu::{
    color_black, color_orange, color_red, color_white, color_yellow, listbar_color,
    menu_buzz_sound, menu_in_sound, menu_move_sound, menu_null_sound, menu_out_sound,
    menu_text_color, text_color_disabled, text_color_highlight, text_color_normal,
    weaponChangeSound, Bitmap_Draw, Bitmap_Init, Menu_AddItem, Menu_Cache, Menu_DefaultKey,
    Menu_Draw, Menu_ItemAtCursor, Menu_SetCursor, Menu_SetCursorToItem, ScrollList_Key,
};
use ui_removebots::{UI_RemoveBotsMenu, UI_RemoveBots_Cache};
use ui_serverinfo::{ServerInfo_Cache, UI_ServerInfoMenu};
use ui_servers2::{punkbuster_items, ArenaServers_Cache, UI_ArenaServersMenu};
use ui_setup::{UI_SetupMenu, UI_SetupMenu_Cache};
use ui_sound::{UI_SoundOptionsMenu, UI_SoundOptionsMenu_Cache};
use ui_sparena::UI_SPArena_Start;
use ui_specifyserver::{SpecifyServer_Cache, UI_SpecifyServerMenu};
use ui_splevel::{UI_SPLevelMenu, UI_SPLevelMenu_Cache, UI_SPLevelMenu_ReInit, UI_SPLevelMenu_f};
use ui_sppostgame::{
    ui_medalPicNames, ui_medalSounds, UI_SPPostgameMenu_Cache, UI_SPPostgameMenu_f,
};
use ui_spskill::{UI_SPSkillMenu, UI_SPSkillMenu_Cache};
use ui_startserver::{
    ServerOptions_Cache, StartServer_Cache, UI_BotSelectMenu_Cache, UI_StartServerMenu,
};
use ui_team::{TeamMain_Cache, UI_TeamMainMenu};
use ui_teamorders::{UI_TeamOrdersMenu, UI_TeamOrdersMenu_f};
use ui_video::{DriverInfo_Cache, GraphicsOptions_Cache, UI_GraphicsOptionsMenu};
extern crate libc;

#[no_mangle]
pub unsafe extern "C" fn UI_DrawPlayer(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut w: libc::c_float,
    mut h: libc::c_float,
    mut pi: *mut playerInfo_t,
    mut time: libc::c_int,
) {
    let mut refdef: refdef_t = refdef_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        fov_x: 0.,
        fov_y: 0.,
        vieworg: [0.; 3],
        viewaxis: [[0.; 3]; 3],
        time: 0,
        rdflags: 0,
        areamask: [0; 32],
        text: [[0; 32]; 8],
    };
    let mut legs: refEntity_t = refEntity_t {
        reType: RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    let mut torso: refEntity_t = refEntity_t {
        reType: RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    let mut head: refEntity_t = refEntity_t {
        reType: RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    let mut gun: refEntity_t = refEntity_t {
        reType: RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    let mut barrel: refEntity_t = refEntity_t {
        reType: RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    let mut flash: refEntity_t = refEntity_t {
        reType: RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    let mut origin: vec3_t = [0.; 3];
    let mut renderfx: libc::c_int = 0;
    let mut mins: vec3_t = [-16i32 as vec_t, -16i32 as vec_t, -24i32 as vec_t];
    let mut maxs: vec3_t = [16i32 as vec_t, 16i32 as vec_t, 32i32 as vec_t];
    let mut len: libc::c_float = 0.;
    let mut xx: libc::c_float = 0.;
    if 0 == (*pi).legsModel
        || 0 == (*pi).torsoModel
        || 0 == (*pi).headModel
        || 0 == (*pi).animations[0usize].numFrames
    {
        return;
    }
    dp_realtime = time;
    if (*pi).pendingWeapon as libc::c_uint != WP_NUM_WEAPONS as libc::c_int as libc::c_uint
        && dp_realtime > (*pi).weaponTimer
    {
        (*pi).weapon = (*pi).pendingWeapon;
        (*pi).lastWeapon = (*pi).pendingWeapon;
        (*pi).pendingWeapon = WP_NUM_WEAPONS;
        (*pi).weaponTimer = 0i32;
        if (*pi).currentWeapon as libc::c_uint != (*pi).weapon as libc::c_uint {
            trap_S_StartLocalSound(weaponChangeSound, CHAN_LOCAL as libc::c_int);
        }
    }
    UI_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    y -= jumpHeight;
    memset(
        &mut refdef as *mut refdef_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refdef_t>() as libc::c_ulong,
    );
    memset(
        &mut legs as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    memset(
        &mut torso as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    memset(
        &mut head as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    refdef.rdflags = 0x1i32;
    AxisClear(refdef.viewaxis.as_mut_ptr());
    refdef.x = x as libc::c_int;
    refdef.y = y as libc::c_int;
    refdef.width = w as libc::c_int;
    refdef.height = h as libc::c_int;
    refdef.fov_x = (refdef.width as libc::c_float / uis.xscale / 640.0f32 * 90.0f32) as libc::c_int
        as libc::c_float;
    xx = ((refdef.width as libc::c_float / uis.xscale) as libc::c_double
        / tan(
            (refdef.fov_x / 360i32 as libc::c_float) as libc::c_double * 3.14159265358979323846f64
        )) as libc::c_float;
    refdef.fov_y = atan2(
        (refdef.height as libc::c_float / uis.yscale) as libc::c_double,
        xx as libc::c_double,
    ) as libc::c_float;
    refdef.fov_y = (refdef.fov_y as libc::c_double
        * (360i32 as libc::c_double / 3.14159265358979323846f64))
        as libc::c_float;
    len = (0.7f64 * (maxs[2usize] - mins[2usize]) as libc::c_double) as libc::c_float;
    origin[0usize] = (len as libc::c_double
        / tan(refdef.fov_x as libc::c_double * 3.14159265358979323846f64
            / 180.0f32 as libc::c_double
            * 0.5f64)) as vec_t;
    origin[1usize] = (0.5f64 * (mins[1usize] + maxs[1usize]) as libc::c_double) as vec_t;
    origin[2usize] = (-0.5f64 * (mins[2usize] + maxs[2usize]) as libc::c_double) as vec_t;
    refdef.time = dp_realtime;
    trap_R_ClearScene();
    UI_PlayerAngles(
        pi,
        legs.axis.as_mut_ptr(),
        torso.axis.as_mut_ptr(),
        head.axis.as_mut_ptr(),
    );
    UI_PlayerAnimation(
        pi,
        &mut legs.oldframe,
        &mut legs.frame,
        &mut legs.backlerp,
        &mut torso.oldframe,
        &mut torso.frame,
        &mut torso.backlerp,
    );
    renderfx = 0x80i32 | 0x40i32;
    legs.hModel = (*pi).legsModel;
    legs.customSkin = (*pi).legsSkin;
    legs.origin[0usize] = origin[0usize];
    legs.origin[1usize] = origin[1usize];
    legs.origin[2usize] = origin[2usize];
    legs.lightingOrigin[0usize] = origin[0usize];
    legs.lightingOrigin[1usize] = origin[1usize];
    legs.lightingOrigin[2usize] = origin[2usize];
    legs.renderfx = renderfx;
    legs.oldorigin[0usize] = legs.origin[0usize];
    legs.oldorigin[1usize] = legs.origin[1usize];
    legs.oldorigin[2usize] = legs.origin[2usize];
    trap_R_AddRefEntityToScene(&mut legs);
    if 0 == legs.hModel {
        return;
    }
    torso.hModel = (*pi).torsoModel;
    if 0 == torso.hModel {
        return;
    }
    torso.customSkin = (*pi).torsoSkin;
    torso.lightingOrigin[0usize] = origin[0usize];
    torso.lightingOrigin[1usize] = origin[1usize];
    torso.lightingOrigin[2usize] = origin[2usize];
    UI_PositionRotatedEntityOnTag(
        &mut torso,
        &mut legs,
        (*pi).legsModel,
        b"tag_torso\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    torso.renderfx = renderfx;
    trap_R_AddRefEntityToScene(&mut torso);
    head.hModel = (*pi).headModel;
    if 0 == head.hModel {
        return;
    }
    head.customSkin = (*pi).headSkin;
    head.lightingOrigin[0usize] = origin[0usize];
    head.lightingOrigin[1usize] = origin[1usize];
    head.lightingOrigin[2usize] = origin[2usize];
    UI_PositionRotatedEntityOnTag(
        &mut head,
        &mut torso,
        (*pi).torsoModel,
        b"tag_head\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    head.renderfx = renderfx;
    trap_R_AddRefEntityToScene(&mut head);
    if (*pi).currentWeapon as libc::c_uint != WP_NONE as libc::c_int as libc::c_uint {
        memset(
            &mut gun as *mut refEntity_t as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
        );
        gun.hModel = (*pi).weaponModel;
        if (*pi).currentWeapon as libc::c_uint == WP_RAILGUN as libc::c_int as libc::c_uint {
            gun.shaderRGBA[0usize] = (*pi).c1RGBA[0usize];
            gun.shaderRGBA[1usize] = (*pi).c1RGBA[1usize];
            gun.shaderRGBA[2usize] = (*pi).c1RGBA[2usize];
            gun.shaderRGBA[3usize] = (*pi).c1RGBA[3usize]
        } else {
            gun.shaderRGBA[0usize] = colorWhite[0usize] as byte;
            gun.shaderRGBA[1usize] = colorWhite[1usize] as byte;
            gun.shaderRGBA[2usize] = colorWhite[2usize] as byte;
            gun.shaderRGBA[3usize] = colorWhite[3usize] as byte
        }
        gun.lightingOrigin[0usize] = origin[0usize];
        gun.lightingOrigin[1usize] = origin[1usize];
        gun.lightingOrigin[2usize] = origin[2usize];
        UI_PositionEntityOnTag(
            &mut gun,
            &mut torso,
            (*pi).torsoModel,
            b"tag_weapon\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        gun.renderfx = renderfx;
        trap_R_AddRefEntityToScene(&mut gun);
    }
    if (*pi).realWeapon == WP_MACHINEGUN as libc::c_int
        || (*pi).realWeapon == WP_GAUNTLET as libc::c_int
        || (*pi).realWeapon == WP_BFG as libc::c_int
    {
        let mut angles: vec3_t = [0.; 3];
        memset(
            &mut barrel as *mut refEntity_t as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
        );
        barrel.lightingOrigin[0usize] = origin[0usize];
        barrel.lightingOrigin[1usize] = origin[1usize];
        barrel.lightingOrigin[2usize] = origin[2usize];
        barrel.renderfx = renderfx;
        barrel.hModel = (*pi).barrelModel;
        angles[1usize] = 0i32 as vec_t;
        angles[0usize] = 0i32 as vec_t;
        angles[2usize] = UI_MachinegunSpinAngle(pi);
        AnglesToAxis(
            angles.as_mut_ptr() as *const vec_t,
            barrel.axis.as_mut_ptr(),
        );
        UI_PositionRotatedEntityOnTag(
            &mut barrel,
            &mut gun,
            (*pi).weaponModel,
            b"tag_barrel\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        trap_R_AddRefEntityToScene(&mut barrel);
    }
    if dp_realtime <= (*pi).muzzleFlashTime {
        if 0 != (*pi).flashModel {
            memset(
                &mut flash as *mut refEntity_t as *mut libc::c_void,
                0i32,
                ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
            );
            flash.hModel = (*pi).flashModel;
            if (*pi).currentWeapon as libc::c_uint == WP_RAILGUN as libc::c_int as libc::c_uint {
                flash.shaderRGBA[0usize] = (*pi).c1RGBA[0usize];
                flash.shaderRGBA[1usize] = (*pi).c1RGBA[1usize];
                flash.shaderRGBA[2usize] = (*pi).c1RGBA[2usize];
                flash.shaderRGBA[3usize] = (*pi).c1RGBA[3usize]
            } else {
                flash.shaderRGBA[0usize] = colorWhite[0usize] as byte;
                flash.shaderRGBA[1usize] = colorWhite[1usize] as byte;
                flash.shaderRGBA[2usize] = colorWhite[2usize] as byte;
                flash.shaderRGBA[3usize] = colorWhite[3usize] as byte
            }
            flash.lightingOrigin[0usize] = origin[0usize];
            flash.lightingOrigin[1usize] = origin[1usize];
            flash.lightingOrigin[2usize] = origin[2usize];
            UI_PositionEntityOnTag(
                &mut flash,
                &mut gun,
                (*pi).weaponModel,
                b"tag_flash\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            flash.renderfx = renderfx;
            trap_R_AddRefEntityToScene(&mut flash);
        }
        if 0. != (*pi).flashDlightColor[0usize]
            || 0. != (*pi).flashDlightColor[1usize]
            || 0. != (*pi).flashDlightColor[2usize]
        {
            trap_R_AddLightToScene(
                flash.origin.as_mut_ptr() as *const vec_t,
                (200i32 + (rand() & 31i32)) as libc::c_float,
                (*pi).flashDlightColor[0usize],
                (*pi).flashDlightColor[1usize],
                (*pi).flashDlightColor[2usize],
            );
        }
    }
    if 0 != (*pi).chat as u64 {
        UI_PlayerFloatSprite(
            pi,
            origin.as_mut_ptr(),
            trap_R_RegisterShaderNoMip(b"sprites/balloon3\x00" as *const u8 as *const libc::c_char),
        );
    }
    origin[0usize] -= 100i32 as libc::c_float;
    origin[1usize] += 100i32 as libc::c_float;
    origin[2usize] += 100i32 as libc::c_float;
    trap_R_AddLightToScene(
        origin.as_mut_ptr() as *const vec_t,
        500i32 as libc::c_float,
        1.0f64 as libc::c_float,
        1.0f64 as libc::c_float,
        1.0f64 as libc::c_float,
    );
    origin[0usize] -= 100i32 as libc::c_float;
    origin[1usize] -= 100i32 as libc::c_float;
    origin[2usize] -= 100i32 as libc::c_float;
    trap_R_AddLightToScene(
        origin.as_mut_ptr() as *const vec_t,
        500i32 as libc::c_float,
        1.0f64 as libc::c_float,
        0.0f64 as libc::c_float,
        0.0f64 as libc::c_float,
    );
    trap_R_RenderScene(&mut refdef);
}
/*
===============
UI_PlayerFloatSprite
===============
*/
unsafe extern "C" fn UI_PlayerFloatSprite(
    mut pi: *mut playerInfo_t,
    mut origin: *mut vec_t,
    mut shader: qhandle_t,
) {
    let mut ent: refEntity_t = refEntity_t {
        reType: RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    memset(
        &mut ent as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    ent.origin[0usize] = *origin.offset(0isize);
    ent.origin[1usize] = *origin.offset(1isize);
    ent.origin[2usize] = *origin.offset(2isize);
    ent.origin[2usize] += 48i32 as libc::c_float;
    ent.reType = RT_SPRITE;
    ent.customShader = shader;
    ent.radius = 10i32 as libc::c_float;
    ent.renderfx = 0i32;
    trap_R_AddRefEntityToScene(&mut ent);
}
/*
======================
UI_PositionEntityOnTag
======================
*/
unsafe extern "C" fn UI_PositionEntityOnTag(
    mut entity: *mut refEntity_t,
    mut parent: *const refEntity_t,
    mut parentModel: clipHandle_t,
    mut tagName: *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut lerped: orientation_t = orientation_t {
        origin: [0.; 3],
        axis: [[0.; 3]; 3],
    };
    trap_CM_LerpTag(
        &mut lerped,
        parentModel,
        (*parent).oldframe,
        (*parent).frame,
        (1.0f64 - (*parent).backlerp as libc::c_double) as libc::c_float,
        tagName,
    );
    (*entity).origin[0usize] = (*parent).origin[0usize];
    (*entity).origin[1usize] = (*parent).origin[1usize];
    (*entity).origin[2usize] = (*parent).origin[2usize];
    i = 0i32;
    while i < 3i32 {
        (*entity).origin[0usize] = (*entity).origin[0usize]
            + (*parent).axis[i as usize][0usize] * lerped.origin[i as usize];
        (*entity).origin[1usize] = (*entity).origin[1usize]
            + (*parent).axis[i as usize][1usize] * lerped.origin[i as usize];
        (*entity).origin[2usize] = (*entity).origin[2usize]
            + (*parent).axis[i as usize][2usize] * lerped.origin[i as usize];
        i += 1
    }
    MatrixMultiply(
        lerped.axis.as_mut_ptr(),
        (*(parent as *mut refEntity_t)).axis.as_mut_ptr(),
        (*entity).axis.as_mut_ptr(),
    );
    (*entity).backlerp = (*parent).backlerp;
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
//
// ui_players.c
static mut dp_realtime: libc::c_int = 0;
/*
======================
UI_PositionRotatedEntityOnTag
======================
*/
unsafe extern "C" fn UI_PositionRotatedEntityOnTag(
    mut entity: *mut refEntity_t,
    mut parent: *const refEntity_t,
    mut parentModel: clipHandle_t,
    mut tagName: *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut lerped: orientation_t = orientation_t {
        origin: [0.; 3],
        axis: [[0.; 3]; 3],
    };
    let mut tempAxis: [vec3_t; 3] = [[0.; 3]; 3];
    trap_CM_LerpTag(
        &mut lerped,
        parentModel,
        (*parent).oldframe,
        (*parent).frame,
        (1.0f64 - (*parent).backlerp as libc::c_double) as libc::c_float,
        tagName,
    );
    (*entity).origin[0usize] = (*parent).origin[0usize];
    (*entity).origin[1usize] = (*parent).origin[1usize];
    (*entity).origin[2usize] = (*parent).origin[2usize];
    i = 0i32;
    while i < 3i32 {
        (*entity).origin[0usize] = (*entity).origin[0usize]
            + (*parent).axis[i as usize][0usize] * lerped.origin[i as usize];
        (*entity).origin[1usize] = (*entity).origin[1usize]
            + (*parent).axis[i as usize][1usize] * lerped.origin[i as usize];
        (*entity).origin[2usize] = (*entity).origin[2usize]
            + (*parent).axis[i as usize][2usize] * lerped.origin[i as usize];
        i += 1
    }
    MatrixMultiply(
        (*entity).axis.as_mut_ptr(),
        lerped.axis.as_mut_ptr(),
        tempAxis.as_mut_ptr(),
    );
    MatrixMultiply(
        tempAxis.as_mut_ptr(),
        (*(parent as *mut refEntity_t)).axis.as_mut_ptr(),
        (*entity).axis.as_mut_ptr(),
    );
}
/*
======================
UI_MachinegunSpinAngle
======================
*/
#[no_mangle]
pub unsafe extern "C" fn UI_MachinegunSpinAngle(mut pi: *mut playerInfo_t) -> libc::c_float {
    let mut delta: libc::c_int = 0;
    let mut angle: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut torsoAnim: libc::c_int = 0;
    delta = dp_realtime - (*pi).barrelTime;
    if 0 != (*pi).barrelSpinning as u64 {
        angle = (*pi).barrelAngle + delta as libc::c_float * 0.9f32
    } else {
        if delta > 1000i32 {
            delta = 1000i32
        }
        speed = (0.5f64
            * (0.9f32 + (1000i32 - delta) as libc::c_float / 1000i32 as libc::c_float)
                as libc::c_double) as libc::c_float;
        angle = (*pi).barrelAngle + delta as libc::c_float * speed
    }
    torsoAnim = (*pi).torsoAnim & !128i32;
    if torsoAnim == TORSO_ATTACK2 as libc::c_int {
        torsoAnim = TORSO_ATTACK as libc::c_int
    }
    if (*pi).barrelSpinning as libc::c_uint
        == !(torsoAnim == TORSO_ATTACK as libc::c_int) as libc::c_int as libc::c_uint
    {
        (*pi).barrelTime = dp_realtime;
        (*pi).barrelAngle = AngleMod(angle);
        (*pi).barrelSpinning = (torsoAnim == TORSO_ATTACK as libc::c_int) as libc::c_int as qboolean
    }
    return angle;
}
/*
===============
UI_PlayerAnimation
===============
*/
unsafe extern "C" fn UI_PlayerAnimation(
    mut pi: *mut playerInfo_t,
    mut legsOld: *mut libc::c_int,
    mut legs: *mut libc::c_int,
    mut legsBackLerp: *mut libc::c_float,
    mut torsoOld: *mut libc::c_int,
    mut torso: *mut libc::c_int,
    mut torsoBackLerp: *mut libc::c_float,
) {
    (*pi).legsAnimationTimer -= uis.frametime;
    if (*pi).legsAnimationTimer < 0i32 {
        (*pi).legsAnimationTimer = 0i32
    }
    UI_LegsSequencing(pi);
    if 0 != (*pi).legs.yawing as libc::c_uint
        && (*pi).legsAnim & !128i32 == LEGS_IDLE as libc::c_int
    {
        UI_RunLerpFrame(pi, &mut (*pi).legs, LEGS_TURN as libc::c_int);
    } else {
        UI_RunLerpFrame(pi, &mut (*pi).legs, (*pi).legsAnim);
    }
    *legsOld = (*pi).legs.oldFrame;
    *legs = (*pi).legs.frame;
    *legsBackLerp = (*pi).legs.backlerp;
    (*pi).torsoAnimationTimer -= uis.frametime;
    if (*pi).torsoAnimationTimer < 0i32 {
        (*pi).torsoAnimationTimer = 0i32
    }
    UI_TorsoSequencing(pi);
    UI_RunLerpFrame(pi, &mut (*pi).torso, (*pi).torsoAnim);
    *torsoOld = (*pi).torso.oldFrame;
    *torso = (*pi).torso.frame;
    *torsoBackLerp = (*pi).torso.backlerp;
}
/*
===============
UI_RunLerpFrame
===============
*/
unsafe extern "C" fn UI_RunLerpFrame(
    mut ci: *mut playerInfo_t,
    mut lf: *mut lerpFrame_t,
    mut newAnimation: libc::c_int,
) {
    let mut f: libc::c_int = 0;
    let mut numFrames: libc::c_int = 0;
    let mut anim: *mut animation_t = 0 as *mut animation_t;
    if newAnimation != (*lf).animationNumber || (*lf).animation.is_null() {
        UI_SetLerpFrameAnimation(ci, lf, newAnimation);
    }
    if dp_realtime >= (*lf).frameTime {
        (*lf).oldFrame = (*lf).frame;
        (*lf).oldFrameTime = (*lf).frameTime;
        anim = (*lf).animation;
        if 0 == (*anim).frameLerp {
            return;
        }
        if dp_realtime < (*lf).animationTime {
            (*lf).frameTime = (*lf).animationTime
        } else {
            (*lf).frameTime = (*lf).oldFrameTime + (*anim).frameLerp
        }
        f = ((*lf).frameTime - (*lf).animationTime) / (*anim).frameLerp;
        numFrames = (*anim).numFrames;
        if 0 != (*anim).flipflop {
            numFrames *= 2i32
        }
        if f >= numFrames {
            f -= numFrames;
            if 0 != (*anim).loopFrames {
                f %= (*anim).loopFrames;
                f += (*anim).numFrames - (*anim).loopFrames
            } else {
                f = numFrames - 1i32;
                (*lf).frameTime = dp_realtime
            }
        }
        if 0 != (*anim).reversed {
            (*lf).frame = (*anim).firstFrame + (*anim).numFrames - 1i32 - f
        } else if 0 != (*anim).flipflop && f >= (*anim).numFrames {
            (*lf).frame = (*anim).firstFrame + (*anim).numFrames - 1i32 - f % (*anim).numFrames
        } else {
            (*lf).frame = (*anim).firstFrame + f
        }
        if dp_realtime > (*lf).frameTime {
            (*lf).frameTime = dp_realtime
        }
    }
    if (*lf).frameTime > dp_realtime + 200i32 {
        (*lf).frameTime = dp_realtime
    }
    if (*lf).oldFrameTime > dp_realtime {
        (*lf).oldFrameTime = dp_realtime
    }
    if (*lf).frameTime == (*lf).oldFrameTime {
        (*lf).backlerp = 0i32 as libc::c_float
    } else {
        (*lf).backlerp = (1.0f64
            - ((dp_realtime - (*lf).oldFrameTime) as libc::c_float
                / ((*lf).frameTime - (*lf).oldFrameTime) as libc::c_float)
                as libc::c_double) as libc::c_float
    };
}
/*
===============
UI_SetLerpFrameAnimation
===============
*/
unsafe extern "C" fn UI_SetLerpFrameAnimation(
    mut ci: *mut playerInfo_t,
    mut lf: *mut lerpFrame_t,
    mut newAnimation: libc::c_int,
) {
    let mut anim: *mut animation_t = 0 as *mut animation_t;
    (*lf).animationNumber = newAnimation;
    newAnimation &= !128i32;
    if newAnimation < 0i32 || newAnimation >= MAX_ANIMATIONS as libc::c_int {
        trap_Error(va(
            b"Bad animation number: %i\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            newAnimation,
        ));
    }
    anim = &mut *(*ci).animations.as_mut_ptr().offset(newAnimation as isize) as *mut animation_t;
    (*lf).animation = anim;
    (*lf).animationTime = (*lf).frameTime + (*anim).initialLerp;
}
/*
===============
UI_TorsoSequencing
===============
*/
unsafe extern "C" fn UI_TorsoSequencing(mut pi: *mut playerInfo_t) {
    let mut currentAnim: libc::c_int = 0;
    currentAnim = (*pi).torsoAnim & !128i32;
    if (*pi).weapon as libc::c_uint != (*pi).currentWeapon as libc::c_uint {
        if currentAnim != TORSO_DROP as libc::c_int {
            (*pi).torsoAnimationTimer = 300i32;
            UI_ForceTorsoAnim(pi, TORSO_DROP as libc::c_int);
        }
    }
    if (*pi).torsoAnimationTimer > 0i32 {
        return;
    }
    if currentAnim == TORSO_GESTURE as libc::c_int {
        UI_SetTorsoAnim(pi, TORSO_STAND as libc::c_int);
        return;
    }
    if currentAnim == TORSO_ATTACK as libc::c_int || currentAnim == TORSO_ATTACK2 as libc::c_int {
        UI_SetTorsoAnim(pi, TORSO_STAND as libc::c_int);
        return;
    }
    if currentAnim == TORSO_DROP as libc::c_int {
        UI_PlayerInfo_SetWeapon(pi, (*pi).weapon);
        (*pi).torsoAnimationTimer = 300i32;
        UI_ForceTorsoAnim(pi, TORSO_RAISE as libc::c_int);
        return;
    }
    if currentAnim == TORSO_RAISE as libc::c_int {
        UI_SetTorsoAnim(pi, TORSO_STAND as libc::c_int);
        return;
    };
}
/*
===============
UI_SetTorsoAnim
===============
*/
unsafe extern "C" fn UI_SetTorsoAnim(mut pi: *mut playerInfo_t, mut anim: libc::c_int) {
    if 0 != (*pi).pendingTorsoAnim {
        anim = (*pi).pendingTorsoAnim;
        (*pi).pendingTorsoAnim = 0i32
    }
    UI_ForceTorsoAnim(pi, anim);
}
/*
===============
UI_ForceTorsoAnim
===============
*/
unsafe extern "C" fn UI_ForceTorsoAnim(mut pi: *mut playerInfo_t, mut anim: libc::c_int) {
    (*pi).torsoAnim = (*pi).torsoAnim & 128i32 ^ 128i32 | anim;
    if anim == TORSO_GESTURE as libc::c_int {
        (*pi).torsoAnimationTimer = 2300i32
    }
    if anim == TORSO_ATTACK as libc::c_int || anim == TORSO_ATTACK2 as libc::c_int {
        (*pi).torsoAnimationTimer = 500i32
    };
}
/*
===============
UI_PlayerInfo_SetWeapon
===============
*/
unsafe extern "C" fn UI_PlayerInfo_SetWeapon(mut pi: *mut playerInfo_t, mut weaponNum: weapon_t) {
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    let mut path: [libc::c_char; 64] = [0; 64];
    (*pi).currentWeapon = weaponNum;
    loop {
        (*pi).realWeapon = weaponNum as libc::c_int;
        (*pi).weaponModel = 0i32;
        (*pi).barrelModel = 0i32;
        (*pi).flashModel = 0i32;
        if weaponNum as libc::c_uint == WP_NONE as libc::c_int as libc::c_uint {
            return;
        }
        item = bg_itemlist.as_mut_ptr().offset(1isize);
        while !(*item).classname.is_null() {
            if !((*item).giType as libc::c_uint != IT_WEAPON as libc::c_int as libc::c_uint) {
                if (*item).giTag as libc::c_uint == weaponNum as libc::c_uint {
                    break;
                }
            }
            item = item.offset(1isize)
        }
        if !(*item).classname.is_null() {
            (*pi).weaponModel = trap_R_RegisterModel((*item).world_model[0usize])
        }
        if !((*pi).weaponModel == 0i32) {
            break;
        }
        if weaponNum as libc::c_uint == WP_MACHINEGUN as libc::c_int as libc::c_uint {
            weaponNum = WP_NONE
        } else {
            weaponNum = WP_MACHINEGUN
        }
    }
    if weaponNum as libc::c_uint == WP_MACHINEGUN as libc::c_int as libc::c_uint
        || weaponNum as libc::c_uint == WP_GAUNTLET as libc::c_int as libc::c_uint
        || weaponNum as libc::c_uint == WP_BFG as libc::c_int as libc::c_uint
    {
        COM_StripExtension(
            (*item).world_model[0usize],
            path.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
        Q_strcat(
            path.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"_barrel.md3\x00" as *const u8 as *const libc::c_char,
        );
        (*pi).barrelModel = trap_R_RegisterModel(path.as_mut_ptr())
    }
    COM_StripExtension(
        (*item).world_model[0usize],
        path.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    Q_strcat(
        path.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"_flash.md3\x00" as *const u8 as *const libc::c_char,
    );
    (*pi).flashModel = trap_R_RegisterModel(path.as_mut_ptr());
    match weaponNum as libc::c_uint {
        1 => {
            (*pi).flashDlightColor[0usize] = 0.6f32;
            (*pi).flashDlightColor[1usize] = 0.6f32;
            (*pi).flashDlightColor[2usize] = 1i32 as vec_t
        }
        2 => {
            (*pi).flashDlightColor[0usize] = 1i32 as vec_t;
            (*pi).flashDlightColor[1usize] = 1i32 as vec_t;
            (*pi).flashDlightColor[2usize] = 0i32 as vec_t
        }
        3 => {
            (*pi).flashDlightColor[0usize] = 1i32 as vec_t;
            (*pi).flashDlightColor[1usize] = 1i32 as vec_t;
            (*pi).flashDlightColor[2usize] = 0i32 as vec_t
        }
        4 => {
            (*pi).flashDlightColor[0usize] = 1i32 as vec_t;
            (*pi).flashDlightColor[1usize] = 0.7f32;
            (*pi).flashDlightColor[2usize] = 0.5f32
        }
        5 => {
            (*pi).flashDlightColor[0usize] = 1i32 as vec_t;
            (*pi).flashDlightColor[1usize] = 0.75f32;
            (*pi).flashDlightColor[2usize] = 0i32 as vec_t
        }
        6 => {
            (*pi).flashDlightColor[0usize] = 0.6f32;
            (*pi).flashDlightColor[1usize] = 0.6f32;
            (*pi).flashDlightColor[2usize] = 1i32 as vec_t
        }
        7 => {
            (*pi).flashDlightColor[0usize] = 1i32 as vec_t;
            (*pi).flashDlightColor[1usize] = 0.5f32;
            (*pi).flashDlightColor[2usize] = 0i32 as vec_t
        }
        8 => {
            (*pi).flashDlightColor[0usize] = 0.6f32;
            (*pi).flashDlightColor[1usize] = 0.6f32;
            (*pi).flashDlightColor[2usize] = 1i32 as vec_t
        }
        9 => {
            (*pi).flashDlightColor[0usize] = 1i32 as vec_t;
            (*pi).flashDlightColor[1usize] = 0.7f32;
            (*pi).flashDlightColor[2usize] = 1i32 as vec_t
        }
        10 => {
            (*pi).flashDlightColor[0usize] = 0.6f32;
            (*pi).flashDlightColor[1usize] = 0.6f32;
            (*pi).flashDlightColor[2usize] = 1i32 as vec_t
        }
        _ => {
            (*pi).flashDlightColor[0usize] = 1i32 as vec_t;
            (*pi).flashDlightColor[1usize] = 1i32 as vec_t;
            (*pi).flashDlightColor[2usize] = 1i32 as vec_t
        }
    };
}
/*
===============
UI_LegsSequencing
===============
*/
unsafe extern "C" fn UI_LegsSequencing(mut pi: *mut playerInfo_t) {
    let mut currentAnim: libc::c_int = 0;
    currentAnim = (*pi).legsAnim & !128i32;
    if (*pi).legsAnimationTimer > 0i32 {
        if currentAnim == LEGS_JUMP as libc::c_int {
            jumpHeight = (56i32 as libc::c_double
                * sin(3.14159265358979323846f64
                    * (1000i32 - (*pi).legsAnimationTimer) as libc::c_double
                    / 1000i32 as libc::c_double)) as libc::c_float
        }
        return;
    }
    if currentAnim == LEGS_JUMP as libc::c_int {
        UI_ForceLegsAnim(pi, LEGS_LAND as libc::c_int);
        (*pi).legsAnimationTimer = 130i32;
        jumpHeight = 0i32 as libc::c_float;
        return;
    }
    if currentAnim == LEGS_LAND as libc::c_int {
        UI_SetLegsAnim(pi, LEGS_IDLE as libc::c_int);
        return;
    };
}
/*
===============
UI_SetLegsAnim
===============
*/
unsafe extern "C" fn UI_SetLegsAnim(mut pi: *mut playerInfo_t, mut anim: libc::c_int) {
    if 0 != (*pi).pendingLegsAnim {
        anim = (*pi).pendingLegsAnim;
        (*pi).pendingLegsAnim = 0i32
    }
    UI_ForceLegsAnim(pi, anim);
}
/*
===============
UI_ForceLegsAnim
===============
*/
unsafe extern "C" fn UI_ForceLegsAnim(mut pi: *mut playerInfo_t, mut anim: libc::c_int) {
    (*pi).legsAnim = (*pi).legsAnim & 128i32 ^ 128i32 | anim;
    if anim == LEGS_JUMP as libc::c_int {
        (*pi).legsAnimationTimer = 1000i32
    };
}
static mut jumpHeight: libc::c_float = 0.;
/*
===============
UI_PlayerAngles
===============
*/
unsafe extern "C" fn UI_PlayerAngles(
    mut pi: *mut playerInfo_t,
    mut legs: *mut vec3_t,
    mut torso: *mut vec3_t,
    mut head: *mut vec3_t,
) {
    let mut legsAngles: vec3_t = [0.; 3];
    let mut torsoAngles: vec3_t = [0.; 3];
    let mut headAngles: vec3_t = [0.; 3];
    let mut dest: libc::c_float = 0.;
    let mut adjust: libc::c_float = 0.;
    headAngles[0usize] = (*pi).viewAngles[0usize];
    headAngles[1usize] = (*pi).viewAngles[1usize];
    headAngles[2usize] = (*pi).viewAngles[2usize];
    headAngles[1usize] = AngleMod(headAngles[1usize]);
    legsAngles[2usize] = 0i32 as vec_t;
    legsAngles[1usize] = legsAngles[2usize];
    legsAngles[0usize] = legsAngles[1usize];
    torsoAngles[2usize] = 0i32 as vec_t;
    torsoAngles[1usize] = torsoAngles[2usize];
    torsoAngles[0usize] = torsoAngles[1usize];
    if (*pi).legsAnim & !128i32 != LEGS_IDLE as libc::c_int
        || (*pi).torsoAnim & !128i32 != TORSO_STAND as libc::c_int
    {
        (*pi).torso.yawing = qtrue;
        (*pi).torso.pitching = qtrue;
        (*pi).legs.yawing = qtrue
    }
    adjust = UI_MovedirAdjustment(pi);
    legsAngles[1usize] = headAngles[1usize] + adjust;
    torsoAngles[1usize] =
        (headAngles[1usize] as libc::c_double + 0.25f64 * adjust as libc::c_double) as vec_t;
    UI_SwingAngles(
        torsoAngles[1usize],
        25i32 as libc::c_float,
        90i32 as libc::c_float,
        0.3f32,
        &mut (*pi).torso.yawAngle,
        &mut (*pi).torso.yawing,
    );
    UI_SwingAngles(
        legsAngles[1usize],
        40i32 as libc::c_float,
        90i32 as libc::c_float,
        0.3f32,
        &mut (*pi).legs.yawAngle,
        &mut (*pi).legs.yawing,
    );
    torsoAngles[1usize] = (*pi).torso.yawAngle;
    legsAngles[1usize] = (*pi).legs.yawAngle;
    if headAngles[0usize] > 180i32 as libc::c_float {
        dest = ((-360i32 as libc::c_float + headAngles[0usize]) as libc::c_double * 0.75f64)
            as libc::c_float
    } else {
        dest = (headAngles[0usize] as libc::c_double * 0.75f64) as libc::c_float
    }
    UI_SwingAngles(
        dest,
        15i32 as libc::c_float,
        30i32 as libc::c_float,
        0.1f32,
        &mut (*pi).torso.pitchAngle,
        &mut (*pi).torso.pitching,
    );
    torsoAngles[0usize] = (*pi).torso.pitchAngle;
    if 0 != (*pi).fixedtorso as u64 {
        torsoAngles[0usize] = 0.0f32
    }
    if 0 != (*pi).fixedlegs as u64 {
        legsAngles[1usize] = torsoAngles[1usize];
        legsAngles[0usize] = 0.0f32;
        legsAngles[2usize] = 0.0f32
    }
    AnglesSubtract(
        headAngles.as_mut_ptr(),
        torsoAngles.as_mut_ptr(),
        headAngles.as_mut_ptr(),
    );
    AnglesSubtract(
        torsoAngles.as_mut_ptr(),
        legsAngles.as_mut_ptr(),
        torsoAngles.as_mut_ptr(),
    );
    AnglesToAxis(legsAngles.as_mut_ptr() as *const vec_t, legs);
    AnglesToAxis(torsoAngles.as_mut_ptr() as *const vec_t, torso);
    AnglesToAxis(headAngles.as_mut_ptr() as *const vec_t, head);
}
/*
==================
UI_SwingAngles
==================
*/
unsafe extern "C" fn UI_SwingAngles(
    mut destination: libc::c_float,
    mut swingTolerance: libc::c_float,
    mut clampTolerance: libc::c_float,
    mut speed: libc::c_float,
    mut angle: *mut libc::c_float,
    mut swinging: *mut qboolean,
) {
    let mut swing: libc::c_float = 0.;
    let mut move_0: libc::c_float = 0.;
    let mut scale: libc::c_float = 0.;
    if 0 == *swinging as u64 {
        swing = AngleSubtract(*angle, destination);
        if swing > swingTolerance || swing < -swingTolerance {
            *swinging = qtrue
        }
    }
    if 0 == *swinging as u64 {
        return;
    }
    swing = AngleSubtract(destination, *angle);
    scale = fabs(swing as libc::c_double) as libc::c_float;
    if (scale as libc::c_double) < swingTolerance as libc::c_double * 0.5f64 {
        scale = 0.5f64 as libc::c_float
    } else if scale < swingTolerance {
        scale = 1.0f64 as libc::c_float
    } else {
        scale = 2.0f64 as libc::c_float
    }
    if swing >= 0i32 as libc::c_float {
        move_0 = uis.frametime as libc::c_float * scale * speed;
        if move_0 >= swing {
            move_0 = swing;
            *swinging = qfalse
        }
        *angle = AngleMod(*angle + move_0)
    } else if swing < 0i32 as libc::c_float {
        move_0 = uis.frametime as libc::c_float * scale * -speed;
        if move_0 <= swing {
            move_0 = swing;
            *swinging = qfalse
        }
        *angle = AngleMod(*angle + move_0)
    }
    swing = AngleSubtract(destination, *angle);
    if swing > clampTolerance {
        *angle = AngleMod(destination - (clampTolerance - 1i32 as libc::c_float))
    } else if swing < -clampTolerance {
        *angle = AngleMod(destination + (clampTolerance - 1i32 as libc::c_float))
    };
}
/*
======================
UI_MovedirAdjustment
======================
*/
unsafe extern "C" fn UI_MovedirAdjustment(mut pi: *mut playerInfo_t) -> libc::c_float {
    let mut relativeAngles: vec3_t = [0.; 3];
    let mut moveVector: vec3_t = [0.; 3];
    relativeAngles[0usize] = (*pi).viewAngles[0usize] - (*pi).moveAngles[0usize];
    relativeAngles[1usize] = (*pi).viewAngles[1usize] - (*pi).moveAngles[1usize];
    relativeAngles[2usize] = (*pi).viewAngles[2usize] - (*pi).moveAngles[2usize];
    AngleVectors(
        relativeAngles.as_mut_ptr() as *const vec_t,
        moveVector.as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
    );
    if (Q_fabs(moveVector[0usize]) as libc::c_double) < 0.01f64 {
        moveVector[0usize] = 0.0f64 as vec_t
    }
    if (Q_fabs(moveVector[1usize]) as libc::c_double) < 0.01f64 {
        moveVector[1usize] = 0.0f64 as vec_t
    }
    if moveVector[1usize] == 0i32 as libc::c_float && moveVector[0usize] > 0i32 as libc::c_float {
        return 0i32 as libc::c_float;
    }
    if moveVector[1usize] < 0i32 as libc::c_float && moveVector[0usize] > 0i32 as libc::c_float {
        return 22i32 as libc::c_float;
    }
    if moveVector[1usize] < 0i32 as libc::c_float && moveVector[0usize] == 0i32 as libc::c_float {
        return 45i32 as libc::c_float;
    }
    if moveVector[1usize] < 0i32 as libc::c_float && moveVector[0usize] < 0i32 as libc::c_float {
        return -22i32 as libc::c_float;
    }
    if moveVector[1usize] == 0i32 as libc::c_float && moveVector[0usize] < 0i32 as libc::c_float {
        return 0i32 as libc::c_float;
    }
    if moveVector[1usize] > 0i32 as libc::c_float && moveVector[0usize] < 0i32 as libc::c_float {
        return 22i32 as libc::c_float;
    }
    if moveVector[1usize] > 0i32 as libc::c_float && moveVector[0usize] == 0i32 as libc::c_float {
        return -45i32 as libc::c_float;
    }
    return -22i32 as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn UI_PlayerInfo_SetModel(
    mut pi: *mut playerInfo_t,
    mut model: *const libc::c_char,
) {
    memset(
        pi as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<playerInfo_t>() as libc::c_ulong,
    );
    UI_RegisterClientModelname(pi, model);
    (*pi).weapon = WP_MACHINEGUN;
    (*pi).currentWeapon = (*pi).weapon;
    (*pi).lastWeapon = (*pi).weapon;
    (*pi).pendingWeapon = WP_NUM_WEAPONS;
    (*pi).weaponTimer = 0i32;
    (*pi).chat = qfalse;
    (*pi).newModel = qtrue;
    UI_PlayerInfo_SetWeapon(pi, (*pi).weapon);
}
#[no_mangle]
pub unsafe extern "C" fn UI_RegisterClientModelname(
    mut pi: *mut playerInfo_t,
    mut modelSkinName: *const libc::c_char,
) -> qboolean {
    let mut modelName: [libc::c_char; 64] = [0; 64];
    let mut skinName: [libc::c_char; 64] = [0; 64];
    let mut filename: [libc::c_char; 64] = [0; 64];
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
    (*pi).torsoModel = 0i32;
    (*pi).headModel = 0i32;
    if 0 == *modelSkinName.offset(0isize) {
        return qfalse;
    }
    Q_strncpyz(
        modelName.as_mut_ptr(),
        modelSkinName,
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    slash = strchr(modelName.as_mut_ptr(), '/' as i32);
    if slash.is_null() {
        Q_strncpyz(
            skinName.as_mut_ptr(),
            b"default\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
    } else {
        Q_strncpyz(
            skinName.as_mut_ptr(),
            slash.offset(1isize),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
        *slash = 0i32 as libc::c_char
    }
    Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"models/players/%s/lower.md3\x00" as *const u8 as *const libc::c_char,
        modelName.as_mut_ptr(),
    );
    (*pi).legsModel = trap_R_RegisterModel(filename.as_mut_ptr());
    if 0 == (*pi).legsModel {
        Com_Printf(
            b"Failed to load model file %s\n\x00" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
        return qfalse;
    }
    Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"models/players/%s/upper.md3\x00" as *const u8 as *const libc::c_char,
        modelName.as_mut_ptr(),
    );
    (*pi).torsoModel = trap_R_RegisterModel(filename.as_mut_ptr());
    if 0 == (*pi).torsoModel {
        Com_Printf(
            b"Failed to load model file %s\n\x00" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
        return qfalse;
    }
    Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"models/players/%s/head.md3\x00" as *const u8 as *const libc::c_char,
        modelName.as_mut_ptr(),
    );
    (*pi).headModel = trap_R_RegisterModel(filename.as_mut_ptr());
    if 0 == (*pi).headModel {
        Com_Printf(
            b"Failed to load model file %s\n\x00" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
        return qfalse;
    }
    if 0 == UI_RegisterClientSkin(pi, modelName.as_mut_ptr(), skinName.as_mut_ptr()) as u64 {
        if 0 == UI_RegisterClientSkin(
            pi,
            modelName.as_mut_ptr(),
            b"default\x00" as *const u8 as *const libc::c_char,
        ) as u64
        {
            Com_Printf(
                b"Failed to load skin file: %s : %s\n\x00" as *const u8 as *const libc::c_char,
                modelName.as_mut_ptr(),
                skinName.as_mut_ptr(),
            );
            return qfalse;
        }
    }
    Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"models/players/%s/animation.cfg\x00" as *const u8 as *const libc::c_char,
        modelName.as_mut_ptr(),
    );
    if 0 == UI_ParseAnimationFile(filename.as_mut_ptr(), pi) as u64 {
        Com_Printf(
            b"Failed to load animation file %s\n\x00" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
        return qfalse;
    }
    return qtrue;
}
/*
======================
UI_ParseAnimationFile
======================
*/
unsafe extern "C" fn UI_ParseAnimationFile(
    mut filename: *const libc::c_char,
    mut pi: *mut playerInfo_t,
) -> qboolean {
    let mut text_p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prev: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fps: libc::c_float = 0.;
    let mut skip: libc::c_int = 0;
    let mut text: [libc::c_char; 20000] = [0; 20000];
    let mut f: fileHandle_t = 0;
    let mut animations: *mut animation_t = 0 as *mut animation_t;
    animations = (*pi).animations.as_mut_ptr();
    memset(
        animations as *mut libc::c_void,
        0i32,
        (::std::mem::size_of::<animation_t>() as libc::c_ulong)
            .wrapping_mul(MAX_ANIMATIONS as libc::c_int as libc::c_ulong),
    );
    (*pi).fixedlegs = qfalse;
    (*pi).fixedtorso = qfalse;
    len = trap_FS_FOpenFile(filename, &mut f, FS_READ);
    if len <= 0i32 {
        return qfalse;
    }
    if len as libc::c_ulong
        >= (::std::mem::size_of::<[libc::c_char; 20000]>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
    {
        Com_Printf(
            b"File %s too long\n\x00" as *const u8 as *const libc::c_char,
            filename,
        );
        trap_FS_FCloseFile(f);
        return qfalse;
    }
    trap_FS_Read(text.as_mut_ptr() as *mut libc::c_void, len, f);
    text[len as usize] = 0i32 as libc::c_char;
    trap_FS_FCloseFile(f);
    text_p = text.as_mut_ptr();
    skip = 0i32;
    loop {
        prev = text_p;
        token = COM_Parse(&mut text_p);
        if 0 == *token.offset(0isize) {
            break;
        }
        if 0 == Q_stricmp(token, b"footsteps\x00" as *const u8 as *const libc::c_char) {
            token = COM_Parse(&mut text_p);
            if 0 == *token.offset(0isize) {
                break;
            }
        } else if 0 == Q_stricmp(token, b"headoffset\x00" as *const u8 as *const libc::c_char) {
            i = 0i32;
            while i < 3i32 {
                token = COM_Parse(&mut text_p);
                if 0 == *token.offset(0isize) {
                    break;
                }
                i += 1
            }
        } else if 0 == Q_stricmp(token, b"sex\x00" as *const u8 as *const libc::c_char) {
            token = COM_Parse(&mut text_p);
            if 0 == *token.offset(0isize) {
                break;
            }
        } else if 0 == Q_stricmp(token, b"fixedlegs\x00" as *const u8 as *const libc::c_char) {
            (*pi).fixedlegs = qtrue
        } else if 0 == Q_stricmp(token, b"fixedtorso\x00" as *const u8 as *const libc::c_char) {
            (*pi).fixedtorso = qtrue
        } else if *token.offset(0isize) as libc::c_int >= '0' as i32
            && *token.offset(0isize) as libc::c_int <= '9' as i32
        {
            text_p = prev;
            break;
        } else {
            Com_Printf(
                b"unknown token \'%s\' in %s\n\x00" as *const u8 as *const libc::c_char,
                token,
                filename,
            );
        }
    }
    i = 0i32;
    while i < MAX_ANIMATIONS as libc::c_int {
        token = COM_Parse(&mut text_p);
        if 0 == *token.offset(0isize) {
            if !(i >= TORSO_GETFLAG as libc::c_int && i <= TORSO_NEGATIVE as libc::c_int) {
                break;
            }
            (*animations.offset(i as isize)).firstFrame =
                (*animations.offset(TORSO_GESTURE as libc::c_int as isize)).firstFrame;
            (*animations.offset(i as isize)).frameLerp =
                (*animations.offset(TORSO_GESTURE as libc::c_int as isize)).frameLerp;
            (*animations.offset(i as isize)).initialLerp =
                (*animations.offset(TORSO_GESTURE as libc::c_int as isize)).initialLerp;
            (*animations.offset(i as isize)).loopFrames =
                (*animations.offset(TORSO_GESTURE as libc::c_int as isize)).loopFrames;
            (*animations.offset(i as isize)).numFrames =
                (*animations.offset(TORSO_GESTURE as libc::c_int as isize)).numFrames;
            (*animations.offset(i as isize)).reversed = qfalse as libc::c_int;
            (*animations.offset(i as isize)).flipflop = qfalse as libc::c_int
        } else {
            (*animations.offset(i as isize)).firstFrame = atoi(token);
            if i == LEGS_WALKCR as libc::c_int {
                skip = (*animations.offset(LEGS_WALKCR as libc::c_int as isize)).firstFrame
                    - (*animations.offset(TORSO_GESTURE as libc::c_int as isize)).firstFrame
            }
            if i >= LEGS_WALKCR as libc::c_int && i < TORSO_GETFLAG as libc::c_int {
                (*animations.offset(i as isize)).firstFrame -= skip
            }
            token = COM_Parse(&mut text_p);
            if 0 == *token.offset(0isize) {
                break;
            }
            (*animations.offset(i as isize)).numFrames = atoi(token);
            (*animations.offset(i as isize)).reversed = qfalse as libc::c_int;
            (*animations.offset(i as isize)).flipflop = qfalse as libc::c_int;
            if (*animations.offset(i as isize)).numFrames < 0i32 {
                (*animations.offset(i as isize)).numFrames =
                    -(*animations.offset(i as isize)).numFrames;
                (*animations.offset(i as isize)).reversed = qtrue as libc::c_int
            }
            token = COM_Parse(&mut text_p);
            if 0 == *token.offset(0isize) {
                break;
            }
            (*animations.offset(i as isize)).loopFrames = atoi(token);
            token = COM_Parse(&mut text_p);
            if 0 == *token.offset(0isize) {
                break;
            }
            fps = atof(token) as libc::c_float;
            if fps == 0i32 as libc::c_float {
                fps = 1i32 as libc::c_float
            }
            (*animations.offset(i as isize)).frameLerp =
                (1000i32 as libc::c_float / fps) as libc::c_int;
            (*animations.offset(i as isize)).initialLerp =
                (1000i32 as libc::c_float / fps) as libc::c_int
        }
        i += 1
    }
    if i != MAX_ANIMATIONS as libc::c_int {
        Com_Printf(
            b"Error parsing animation file: %s\n\x00" as *const u8 as *const libc::c_char,
            filename,
        );
        return qfalse;
    }
    return qtrue;
}
/*
==========================
UI_RegisterClientSkin
==========================
*/
unsafe extern "C" fn UI_RegisterClientSkin(
    mut pi: *mut playerInfo_t,
    mut modelName: *const libc::c_char,
    mut skinName: *const libc::c_char,
) -> qboolean {
    let mut filename: [libc::c_char; 64] = [0; 64];
    Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"models/players/%s/lower_%s.skin\x00" as *const u8 as *const libc::c_char,
        modelName,
        skinName,
    );
    (*pi).legsSkin = trap_R_RegisterSkin(filename.as_mut_ptr());
    Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"models/players/%s/upper_%s.skin\x00" as *const u8 as *const libc::c_char,
        modelName,
        skinName,
    );
    (*pi).torsoSkin = trap_R_RegisterSkin(filename.as_mut_ptr());
    Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"models/players/%s/head_%s.skin\x00" as *const u8 as *const libc::c_char,
        modelName,
        skinName,
    );
    (*pi).headSkin = trap_R_RegisterSkin(filename.as_mut_ptr());
    if 0 == (*pi).legsSkin || 0 == (*pi).torsoSkin || 0 == (*pi).headSkin {
        return qfalse;
    }
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn UI_PlayerInfo_SetInfo(
    mut pi: *mut playerInfo_t,
    mut legsAnim: libc::c_int,
    mut torsoAnim: libc::c_int,
    mut viewAngles: *mut vec_t,
    mut moveAngles: *mut vec_t,
    mut weaponNumber: weapon_t,
    mut chat: qboolean,
) {
    let mut currentAnim: libc::c_int = 0;
    let mut weaponNum: weapon_t = WP_NONE;
    let mut c: libc::c_int = 0;
    (*pi).chat = chat;
    c = trap_Cvar_VariableValue(b"color1\x00" as *const u8 as *const libc::c_char) as libc::c_int;
    (*pi).color1[2usize] = 0i32 as vec_t;
    (*pi).color1[1usize] = (*pi).color1[2usize];
    (*pi).color1[0usize] = (*pi).color1[1usize];
    if c < 1i32 || c > 7i32 {
        (*pi).color1[0usize] = 1i32 as vec_t;
        (*pi).color1[1usize] = 1i32 as vec_t;
        (*pi).color1[2usize] = 1i32 as vec_t
    } else {
        if 0 != c & 1i32 {
            (*pi).color1[2usize] = 1.0f32
        }
        if 0 != c & 2i32 {
            (*pi).color1[1usize] = 1.0f32
        }
        if 0 != c & 4i32 {
            (*pi).color1[0usize] = 1.0f32
        }
    }
    (*pi).c1RGBA[0usize] = (255i32 as libc::c_float * (*pi).color1[0usize]) as byte;
    (*pi).c1RGBA[1usize] = (255i32 as libc::c_float * (*pi).color1[1usize]) as byte;
    (*pi).c1RGBA[2usize] = (255i32 as libc::c_float * (*pi).color1[2usize]) as byte;
    (*pi).c1RGBA[3usize] = 255i32 as byte;
    (*pi).viewAngles[0usize] = *viewAngles.offset(0isize);
    (*pi).viewAngles[1usize] = *viewAngles.offset(1isize);
    (*pi).viewAngles[2usize] = *viewAngles.offset(2isize);
    (*pi).moveAngles[0usize] = *moveAngles.offset(0isize);
    (*pi).moveAngles[1usize] = *moveAngles.offset(1isize);
    (*pi).moveAngles[2usize] = *moveAngles.offset(2isize);
    if 0 != (*pi).newModel as u64 {
        (*pi).newModel = qfalse;
        jumpHeight = 0i32 as libc::c_float;
        (*pi).pendingLegsAnim = 0i32;
        UI_ForceLegsAnim(pi, legsAnim);
        (*pi).legs.yawAngle = *viewAngles.offset(1isize);
        (*pi).legs.yawing = qfalse;
        (*pi).pendingTorsoAnim = 0i32;
        UI_ForceTorsoAnim(pi, torsoAnim);
        (*pi).torso.yawAngle = *viewAngles.offset(1isize);
        (*pi).torso.yawing = qfalse;
        if weaponNumber as libc::c_uint != WP_NUM_WEAPONS as libc::c_int as libc::c_uint {
            (*pi).weapon = weaponNumber;
            (*pi).currentWeapon = weaponNumber;
            (*pi).lastWeapon = weaponNumber;
            (*pi).pendingWeapon = WP_NUM_WEAPONS;
            (*pi).weaponTimer = 0i32;
            UI_PlayerInfo_SetWeapon(pi, (*pi).weapon);
        }
        return;
    }
    if weaponNumber as libc::c_uint == WP_NUM_WEAPONS as libc::c_int as libc::c_uint {
        (*pi).pendingWeapon = WP_NUM_WEAPONS;
        (*pi).weaponTimer = 0i32
    } else if weaponNumber as libc::c_uint != WP_NONE as libc::c_int as libc::c_uint {
        (*pi).pendingWeapon = weaponNumber;
        (*pi).weaponTimer = dp_realtime + 250i32
    }
    weaponNum = (*pi).lastWeapon;
    (*pi).weapon = weaponNum;
    if torsoAnim == BOTH_DEATH1 as libc::c_int || legsAnim == BOTH_DEATH1 as libc::c_int {
        legsAnim = BOTH_DEATH1 as libc::c_int;
        torsoAnim = legsAnim;
        (*pi).currentWeapon = WP_NONE;
        (*pi).weapon = (*pi).currentWeapon;
        UI_PlayerInfo_SetWeapon(pi, (*pi).weapon);
        jumpHeight = 0i32 as libc::c_float;
        (*pi).pendingLegsAnim = 0i32;
        UI_ForceLegsAnim(pi, legsAnim);
        (*pi).pendingTorsoAnim = 0i32;
        UI_ForceTorsoAnim(pi, torsoAnim);
        return;
    }
    currentAnim = (*pi).legsAnim & !128i32;
    if legsAnim != LEGS_JUMP as libc::c_int
        && (currentAnim == LEGS_JUMP as libc::c_int || currentAnim == LEGS_LAND as libc::c_int)
    {
        (*pi).pendingLegsAnim = legsAnim
    } else if legsAnim != currentAnim {
        jumpHeight = 0i32 as libc::c_float;
        (*pi).pendingLegsAnim = 0i32;
        UI_ForceLegsAnim(pi, legsAnim);
    }
    if torsoAnim == TORSO_STAND as libc::c_int || torsoAnim == TORSO_STAND2 as libc::c_int {
        if weaponNum as libc::c_uint == WP_NONE as libc::c_int as libc::c_uint
            || weaponNum as libc::c_uint == WP_GAUNTLET as libc::c_int as libc::c_uint
        {
            torsoAnim = TORSO_STAND2 as libc::c_int
        } else {
            torsoAnim = TORSO_STAND as libc::c_int
        }
    }
    if torsoAnim == TORSO_ATTACK as libc::c_int || torsoAnim == TORSO_ATTACK2 as libc::c_int {
        if weaponNum as libc::c_uint == WP_NONE as libc::c_int as libc::c_uint
            || weaponNum as libc::c_uint == WP_GAUNTLET as libc::c_int as libc::c_uint
        {
            torsoAnim = TORSO_ATTACK2 as libc::c_int
        } else {
            torsoAnim = TORSO_ATTACK as libc::c_int
        }
        (*pi).muzzleFlashTime = dp_realtime + 20i32
    }
    currentAnim = (*pi).torsoAnim & !128i32;
    if weaponNum as libc::c_uint != (*pi).currentWeapon as libc::c_uint
        || currentAnim == TORSO_RAISE as libc::c_int
        || currentAnim == TORSO_DROP as libc::c_int
    {
        (*pi).pendingTorsoAnim = torsoAnim
    } else if (currentAnim == TORSO_GESTURE as libc::c_int
        || currentAnim == TORSO_ATTACK as libc::c_int)
        && torsoAnim != currentAnim
    {
        (*pi).pendingTorsoAnim = torsoAnim
    } else if torsoAnim != currentAnim {
        (*pi).pendingTorsoAnim = 0i32;
        UI_ForceTorsoAnim(pi, torsoAnim);
    };
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct playersettings_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub player: menubitmap_s,
    pub name: menufield_s,
    pub handicap: menulist_s,
    pub effects: menulist_s,
    pub back: menubitmap_s,
    pub model: menubitmap_s,
    pub item_null: menubitmap_s,
    pub fxBasePic: qhandle_t,
    pub fxPic: [qhandle_t; 7],
    pub playerinfo: playerInfo_t,
    pub current_fx: libc::c_int,
    pub playerModel: [libc::c_char; 64],
}

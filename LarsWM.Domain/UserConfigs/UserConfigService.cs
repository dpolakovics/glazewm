﻿namespace LarsWM.Domain.UserConfigs
{
    // TODO: Remove properties from this class once WindowsApiFacade & WindowsApiService use DI
    public class UserConfigService
    {
        public UserConfig UserConfig { get; set; } = null;

        // TODO: allow regular expressions
        // eg. for WMP's "now playing" toolbar: StartsWith("WMP9MediaBarFlyout"))
        public static string[] WindowClassesToIgnore = new string[] {
            // Tray on primary screen
            "Shell_TrayWnd",
            // Trays on secondary screens
            "Shell_SecondaryTrayWnd",
            // ??
            "TaskManagerWindow",
            // Microsoft Text Framework service IME
            "MSCTFIME UI",
            // Desktop window (holds wallpaper & desktop icons)
            "SHELLDLL_DefView",
            // Background for lock screen
            "LockScreenBackstopFrame",
            // ??
            "Progman",
            // Windows 7 open Start Menu
            "DV2ControlHost",
            // Some Windows 8 thing
            "Shell_CharmWindow",

            /* 
             * Consider adding: 
             * "MsgrIMEWindowClass", // Window live messenger notification
             * "SysShadow", // Windows live messenger shadow-hack
             * "Button", // UI component, e.g. Start Menu button
             * "Frame Alternate Owner", // Edge
             * "MultitaskingViewFrame", // The original Win + Tab view
             */
        };

        public static string[] ProcessNamesToIgnore = new string[] {
            "SearchUI",
            "ShellExperienceHost",
            "LockApp",
            "PeopleExperienceHost",
            "StartMenuExperienceHost",
        };

        public static int InnerGap = 20;
        public static int OuterGap = 20;
    }
}
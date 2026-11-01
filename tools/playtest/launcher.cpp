// ReGTA Chinatown Wars — Windows Playtest Launcher
// Lanceur PC autonome pour les sessions de test manuel.

#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <shellapi.h>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
#include <vector>
#include <filesystem>
#include <fstream>
#include <iostream>

namespace fs = std::filesystem;

static std::string get_exe_dir() {
    char buf[MAX_PATH];
    GetModuleFileNameA(NULL, buf, MAX_PATH);
    std::string path(buf);
    size_t pos = path.find_last_of("\\/");
    return (pos != std::string::npos) ? path.substr(0, pos) : ".";
}

static std::string trim(const std::string& s) {
    size_t start = s.find_first_not_of(" \t\r\n");
    if (start == std::string::npos) return "";
    size_t end = s.find_last_not_of(" \t\r\n");
    return s.substr(start, end - start + 1);
}

static std::string find_iso_in_ini(const std::string& ini_path) {
    std::ifstream f(ini_path);
    if (!f.is_open()) return "";
    std::string line;
    while (std::getline(f, line)) {
        size_t comment = line.find('#');
        if (comment != std::string::npos) line = line.substr(0, comment);
        size_t eq = line.find('=');
        if (eq == std::string::npos) continue;
        std::string key = trim(line.substr(0, eq));
        std::string val = trim(line.substr(eq + 1));
        if (key == "iso") return val;
    }
    return "";
}

static std::string resolve_iso(const std::string& exe_dir) {
    std::string ini_path = exe_dir + "\\ctw.ini";
    std::string from_ini = find_iso_in_ini(ini_path);
    if (!from_ini.empty()) {
        fs::path p(from_ini);
        if (p.is_relative()) p = fs::path(exe_dir) / p;
        if (fs::exists(p)) return fs::absolute(p).string();
    }

    // Recherche directe d'un fichier .iso dans le dossier courant
    std::error_code ec;
    for (const auto& entry : fs::directory_iterator(exe_dir, ec)) {
        if (entry.is_regular_file() && entry.path().extension() == ".iso") {
            return fs::absolute(entry.path()).string();
        }
    }

    // Recherche dans les emplacements standards du projet
    std::vector<std::string> candidates = {
        exe_dir + "\\..\\..\\work\\Chinatown-Wars-PSP.iso",
        exe_dir + "\\..\\work\\Chinatown-Wars-PSP.iso",
        exe_dir + "\\work\\Chinatown-Wars-PSP.iso",
        exe_dir + "\\Chinatown-Wars-PSP.iso"
    };
    for (const auto& c : candidates) {
        if (fs::exists(c)) return fs::canonical(c).string();
    }

    return "";
}

int main(int argc, char** argv) {
    SetConsoleOutputCP(CP_UTF8);
    SetConsoleCP(CP_UTF8);
    SetConsoleTitleA("ReGTA Chinatown Wars — PC Playtest Edition");

    std::string exe_dir = get_exe_dir();
    SetCurrentDirectoryA(exe_dir.c_str());

    // Création du dossier logs
    std::string logs_dir = exe_dir + "\\logs";
    fs::create_directories(logs_dir);

    // Lecture de version.json si présent
    std::string version = "1.0-Playtest";
    std::string build_date = __DATE__ " " __TIME__;
    std::string bin_id = "bfd36700b9da0d5c0affbff5870bdc8d9adbb19e1b280ea170b28ed941848d4c";
    std::string version_file = exe_dir + "\\version.json";
    if (fs::exists(version_file)) {
        std::ifstream vf(version_file);
        std::string content((std::istreambuf_iterator<char>(vf)), std::istreambuf_iterator<char>());
        // parsing simple des clés principales
        auto get_field = [&](const std::string& key) -> std::string {
            size_t pos = content.find("\"" + key + "\"");
            if (pos == std::string::npos) return "";
            size_t colon = content.find(':', pos);
            if (colon == std::string::npos) return "";
            size_t quote1 = content.find('\"', colon);
            if (quote1 == std::string::npos) return "";
            size_t quote2 = content.find('\"', quote1 + 1);
            if (quote2 == std::string::npos) return "";
            return content.substr(quote1 + 1, quote2 - quote1 - 1);
        };
        std::string v = get_field("version"); if (!v.empty()) version = v;
        std::string d = get_field("build_date"); if (!d.empty()) build_date = d;
        std::string id = get_field("binary_sha256"); if (!id.empty()) bin_id = id;
    }

    std::cout << "================================================================================" << std::endl;
    std::cout << "          ReGTA Chinatown Wars — Edition Playtest PC (PSP Europe ULES-01347)    " << std::endl;
    std::cout << "================================================================================" << std::endl;
    std::cout << "  Version           : " << version << std::endl;
    std::cout << "  Date de build     : " << build_date << std::endl;
    std::cout << "  Identifiant SHA256: " << bin_id << std::endl;
    std::cout << "  Runtime C++       : Clang C++20 MT_StaticRelease + Bink Mutex + FIFO Scheduler" << std::endl;
    std::cout << "================================================================================" << std::endl;
    std::cout << std::endl;

    // Vérification de ctw_game.exe
    std::string game_exe = exe_dir + "\\ctw_game.exe";
    if (!fs::exists(game_exe)) {
        game_exe = exe_dir + "\\ctw.exe";
    }
    if (!fs::exists(game_exe)) {
        std::cerr << "[ERREUR CRITIQUE] Binaire de jeu introuvable dans ce dossier !" << std::endl;
        std::cerr << "Attendu : " << exe_dir << "\\ctw_game.exe" << std::endl;
        std::cerr << "Veuillez executer 'update_playtest.py' ou recopier le binaire valide." << std::endl;
        std::cout << "\nAppuyez sur Entree pour quitter...";
        std::cin.get();
        return 1;
    }

    // Vérification de image.bin
    std::string image_bin = exe_dir + "\\image.bin";
    if (!fs::exists(image_bin)) {
        std::cerr << "[ERREUR CRITIQUE] Table de remappage 'image.bin' introuvable !" << std::endl;
        std::cerr << "Attendu : " << image_bin << std::endl;
        std::cout << "\nAppuyez sur Entree pour quitter...";
        std::cin.get();
        return 1;
    }

    // Détection de l'image ISO
    std::string iso_path = resolve_iso(exe_dir);
    if (iso_path.empty()) {
        std::cerr << "[ERREUR ISO] Image UMD introuvable !" << std::endl;
        std::cerr << "Pour jouer, placez votre fichier 'Chinatown-Wars-PSP.iso' (ULES-01347)" << std::endl;
        std::cerr << "directement dans ce dossier : " << exe_dir << std::endl;
        std::cerr << "ou configurez son chemin dans 'ctw.ini' (ligne : iso = <chemin>)." << std::endl;
        std::cout << "\nAppuyez sur Entree pour quitter...";
        std::cin.get();
        return 1;
    }
    std::cout << "[OK] Image ISO detectee : " << iso_path << std::endl;
    std::cout << "[OK] Binaire ReGTA      : " << game_exe << std::endl;
    std::cout << "[OK] Mapping adresses   : " << image_bin << std::endl;
    std::cout << "[OK] Repertoire travail : " << exe_dir << std::endl;

    // Purge des variables d'environnement de test automatisé
    SetEnvironmentVariableA("CTW_INPUT", NULL);
    SetEnvironmentVariableA("CTW_HEADLESS", NULL);
    SetEnvironmentVariableA("CTW_SHOT_EVERY", NULL);
    SetEnvironmentVariableA("CTW_LOG", "all");

    std::cout << std::endl;
    std::cout << "--------------------------------------------------------------------------------" << std::endl;
    std::cout << "  GUIDE DES COMMANDES DE JEU (Clavier & Manette Xbox)" << std::endl;
    std::cout << "--------------------------------------------------------------------------------" << std::endl;
    std::cout << "  [MANETTE (Recommandee)]" << std::endl;
    std::cout << "    - Stick gauche               : Deplacements (Nage, Marche, Conduite)" << std::endl;
    std::cout << "    - Touche A (Croix PSP)       : Sprint / Valider dialogue / Nage rapide" << std::endl;
    std::cout << "    - Touche B (Rond PSP)        : Tirer / Frapper" << std::endl;
    std::cout << "    - Touche X (Carre PSP)       : Sauter / Frein a main" << std::endl;
    std::cout << "    - Touche Y (Triangle PSP)    : Entrer / Sortir de vehicule" << std::endl;
    std::cout << "    - Gachettes LB / RB (ou LT/RT: Gachettes L / R (Bris de vitre habitacle !)" << std::endl;
    std::cout << "    - Start                      : Menu pause / Passer cinematique" << std::endl;
    std::cout << std::endl;
    std::cout << "  [CLAVIER]" << std::endl;
    std::cout << "    - Deplacements analogiques   : Touches I (Haut), K (Bas), J (Gauche), L (Droite)" << std::endl;
    std::cout << "    - Croix directionnelle       : Fleches directionnelles" << std::endl;
    std::cout << "    - Croix (Sprint / Valider)   : Z" << std::endl;
    std::cout << "    - Rond (Attaque / Tir)       : X" << std::endl;
    std::cout << "    - Carre (Saut)               : A" << std::endl;
    std::cout << "    - Triangle (Vehicule)        : S" << std::endl;
    std::cout << "    - Gachette L / Gachette R    : Q (L) / E (R)" << std::endl;
    std::cout << "    - Start / Select             : Entree / Retour arriere" << std::endl;
    std::cout << "    - Basculer Plein ecran       : F11" << std::endl;
    std::cout << "--------------------------------------------------------------------------------" << std::endl;
    std::cout << std::endl;
    std::cout << "Demarrage du jeu... La fenetre graphique va s'ouvrir." << std::endl;

    // Construction de la ligne de commande
    std::string cmdline = "\"" + game_exe + "\" --iso \"" + iso_path + "\"";

    // Préparation de la redirection des logs vers logs/playtest.log
    std::string log_file_path = logs_dir + "\\playtest.log";
    SECURITY_ATTRIBUTES sa{};
    sa.nLength = sizeof(sa);
    sa.bInheritHandle = TRUE;
    HANDLE hLog = CreateFileA(log_file_path.c_str(), GENERIC_WRITE, FILE_SHARE_READ, &sa, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, NULL);

    STARTUPINFOA si{};
    si.cb = sizeof(si);
    if (hLog != INVALID_HANDLE_VALUE) {
        si.dwFlags |= STARTF_USESTDHANDLES;
        si.hStdOutput = hLog;
        si.hStdError = hLog;
        si.hStdInput = GetStdHandle(STD_INPUT_HANDLE);
    }

    PROCESS_INFORMATION pi{};
    char cmd_buf[2048];
    strncpy_s(cmd_buf, cmdline.c_str(), sizeof(cmd_buf) - 1);

    BOOL created = CreateProcessA(
        NULL,
        cmd_buf,
        NULL,
        NULL,
        TRUE,
        0,
        NULL,
        exe_dir.c_str(),
        &si,
        &pi
    );

    if (hLog != INVALID_HANDLE_VALUE) CloseHandle(hLog);

    if (!created) {
        DWORD err = GetLastError();
        std::cerr << "[ERREUR SYSTEME] Echec de lancement du processus (Code: " << err << ")" << std::endl;
        std::cout << "\nAppuyez sur Entree pour quitter...";
        std::cin.get();
        return 1;
    }

    std::cout << "[JEU EN COURS D'EXECUTION] PID: " << pi.dwProcessId << std::endl;
    std::cout << "Journal enregistre dans : logs/playtest.log" << std::endl;

    // Attente de la fin du processus
    WaitForSingleObject(pi.hProcess, INFINITE);

    DWORD exitCode = 0;
    GetExitCodeProcess(pi.hProcess, &exitCode);
    CloseHandle(pi.hProcess);
    CloseHandle(pi.hThread);

    std::cout << "\n[FIN DE SESSION] Le jeu s'est termine avec le code: " << exitCode << std::endl;
    std::cout << "Toutes vos sauvegardes sont conservees dans : " << exe_dir << "\\ms0" << std::endl;
    std::cout << "Fermeture dans 3 secondes..." << std::endl;
    Sleep(3000);

    return (int)exitCode;
}

# Panduan Kontribusi & Feedback Komunitas MyDiskly 🚀

Terima kasih telah tertarik untuk berkontribusi pada pengembangan **MyDiskly**! Aplikasi ini dikembangkan secara terbuka (*open-source*) di bawah lisensi MIT untuk menyediakan utilitas pemantauan dan pembersih penyimpanan yang cepat, ringan, dan estetik bagi pengguna Windows.

---

## 💡 Bagaimana Cara Memberikan Feedback & Usulan Fitur?

Kami secara berkala memperbarui MyDiskly berdasarkan masukan dari komunitas. Anda dapat memberikan feedback dengan cara:

1. **Membuat Feature Request di GitHub Issues**:
   - Kunjungi [GitHub Issues](https://github.com/darkostepbro/mydiskly-desktop/issues/new/choose).
   - Pilih template **💡 Usulan Fitur Baru / Feature Request**.
   - Berikan deskripsi yang jelas mengenai fitur atau sensor yang Anda inginkan (misalnya: integrasi cloud storage, sensor temperatur SSD NVMe spesifik, atau penyesuaian tema).

2. **Melaporkan Masalah atau Bug**:
   - Jika Anda menemukan crash, glitch tampilan, atau kendala performa, gunakan template **🐛 Laporan Bug / Bug Report**.
   - Sertakan langkah reproduksi dan versi Windows yang Anda gunakan.

---

## 🛠️ Menyiapkan Lingkungan Pengembangan Lokal

Jika Anda ingin memodifikasi kode sumber atau membuat *Pull Request*:

### 1. Prasyarat
- **Windows 10 / 11 (64-bit)**
- **Python 3.10, 3.11, atau 3.12**
- Git

### 2. Kloning & Pemasangan Pustaka
```powershell
# Kloning repository
git clone https://github.com/darkostepbro/mydiskly-desktop.git
cd mydiskly-desktop

# Buat virtual environment (opsional namun disarankan)
python -m venv venv
.\venv\Scripts\Activate.ps1

# Install dependensi
pip install -r requirements.txt
```

### 3. Menjalankan Langsung dari Source
Anda dapat menjalankan aplikasi langsung selama proses pengembangan tanpa perlu membangun ulang binary:
```powershell
python src/main.py
```

### 4. Membangun Binary & Distribusi Portable
Untuk mengompilasi engine DLL dan paket standalone executable:
```powershell
# Kompilasi disktrex.dll
python scripts/build_dll.py

# Kompilasi paket portable lengkap (MyDiskly.exe + lib/)
python scripts/build_exe.py
```
Hasil kompilasi siap pakai akan berada di direktori `dist/mydiskly-v1/`.

---

## 📋 Alur Pengajuan Pull Request (PR)

1. Lakukan *Fork* repository ini.
2. Buat branch baru untuk fitur Anda (`git checkout -b feature/nama-fitur-keren`).
3. Lakukan pengujian secara menyeluruh di Windows (pastikan *Single-Instance Mutex*, *Tray Icon*, dan *DPI Scaling* tetap berjalan stabil).
4. Buat *Commit* dengan pesan yang deskriptif (`git commit -m "feat: tambahkan sensor SMART untuk drive NVMe tambahan"`).
5. Dorong branch Anda ke GitHub (`git push origin feature/nama-fitur-keren`).
6. Buka **Pull Request** ke branch `main`.

---

## 💬 Hubungi Tim Pengembang & Komunitas

- **Publisher**: [Seroja Labs](https://github.com/serojalabs)
- **Author**: Darkostepbro
- **Feedback & Repository**: [github.com/darkostepbro/mydiskly-desktop](https://github.com/darkostepbro/mydiskly-desktop)
- **Lisensi**: MIT License

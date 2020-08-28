use std::{env, fs, io};
use std::path::{Path, PathBuf};
use flate2::read::GzDecoder;
use tar::Archive;
use curl::easy::Easy;
use std::io::Write;


const CBLAS_SRC: &str = "http://www.netlib.org/blas/blast-forum/cblas.tgz";


fn download<P: AsRef<Path>>(source_url: &str, target_file: P) -> anyhow::Result<()> {    
    let f = fs::File::create(&target_file)?;
    let mut writer = io::BufWriter::new(f);
    let mut easy = Easy::new();

    easy.useragent("Curl Download")?;
    easy.url(source_url)?;
    easy.write_function(move |data| Ok(writer.write(data).unwrap()))?;
    easy.perform()?;

    let response_code = easy.response_code()?;
    if response_code == 200 {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Unexpected response code {} for {}",
            response_code,
            source_url
        ))
    }
}

fn extract<P1: AsRef<Path>, P2: AsRef<Path>>(filename: P1, outpath: P2) -> anyhow::Result<()> {
    let file = fs::File::open(&filename)?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);
    archive.unpack(outpath.as_ref())?;
    
    Ok(())
}

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let cblas_name = out_path.join("cblas.tgz");
    let cblas_dir = out_path.join("CBLAS");

    if !cblas_dir.exists() {
        download(CBLAS_SRC, &cblas_name).unwrap();
        extract(cblas_name, &out_path).unwrap();
    }
    
    cc::Build::new()
        .warnings(false)
        .extra_warnings(false)
        .static_flag(true)
        .define("ADD_", None)
        .include(cblas_dir.join("include"))
        .file(cblas_dir.join("src/cblas_caxpy.c"))
        .file(cblas_dir.join("src/cblas_ccopy.c"))
        .file(cblas_dir.join("src/cblas_cdotc_sub.c"))
        .file(cblas_dir.join("src/cblas_cdotu_sub.c"))
        .file(cblas_dir.join("src/cblas_cgbmv.c"))
        .file(cblas_dir.join("src/cblas_cgemm.c"))
        .file(cblas_dir.join("src/cblas_cgemv.c"))
        .file(cblas_dir.join("src/cblas_cgerc.c"))
        .file(cblas_dir.join("src/cblas_cgeru.c"))
        .file(cblas_dir.join("src/cblas_chbmv.c"))
        .file(cblas_dir.join("src/cblas_chemm.c"))
        .file(cblas_dir.join("src/cblas_chemv.c"))
        .file(cblas_dir.join("src/cblas_cher.c"))
        .file(cblas_dir.join("src/cblas_cher2.c"))
        .file(cblas_dir.join("src/cblas_cher2k.c"))
        .file(cblas_dir.join("src/cblas_cherk.c"))
        .file(cblas_dir.join("src/cblas_chpmv.c"))
        .file(cblas_dir.join("src/cblas_chpr.c"))
        .file(cblas_dir.join("src/cblas_chpr2.c"))
        .file(cblas_dir.join("src/cblas_cscal.c"))
        .file(cblas_dir.join("src/cblas_csscal.c"))
        .file(cblas_dir.join("src/cblas_cswap.c"))
        .file(cblas_dir.join("src/cblas_csymm.c"))
        .file(cblas_dir.join("src/cblas_csyr2k.c"))
        .file(cblas_dir.join("src/cblas_csyrk.c"))
        .file(cblas_dir.join("src/cblas_ctbmv.c"))
        .file(cblas_dir.join("src/cblas_ctbsv.c"))
        .file(cblas_dir.join("src/cblas_ctpmv.c"))
        .file(cblas_dir.join("src/cblas_ctpsv.c"))
        .file(cblas_dir.join("src/cblas_ctrmm.c"))
        .file(cblas_dir.join("src/cblas_ctrmv.c"))
        .file(cblas_dir.join("src/cblas_ctrsm.c"))
        .file(cblas_dir.join("src/cblas_ctrsv.c"))
        .file(cblas_dir.join("src/cblas_dasum.c"))
        .file(cblas_dir.join("src/cblas_daxpy.c"))
        .file(cblas_dir.join("src/cblas_dcopy.c"))
        .file(cblas_dir.join("src/cblas_ddot.c"))
        .file(cblas_dir.join("src/cblas_dgbmv.c"))
        .file(cblas_dir.join("src/cblas_dgemm.c"))
        .file(cblas_dir.join("src/cblas_dgemv.c"))
        .file(cblas_dir.join("src/cblas_dger.c"))
        .file(cblas_dir.join("src/cblas_dnrm2.c"))
        .file(cblas_dir.join("src/cblas_drot.c"))
        .file(cblas_dir.join("src/cblas_drotg.c"))
        .file(cblas_dir.join("src/cblas_drotm.c"))
        .file(cblas_dir.join("src/cblas_drotmg.c"))
        .file(cblas_dir.join("src/cblas_dsbmv.c"))
        .file(cblas_dir.join("src/cblas_dscal.c"))
        .file(cblas_dir.join("src/cblas_dsdot.c"))
        .file(cblas_dir.join("src/cblas_dspmv.c"))
        .file(cblas_dir.join("src/cblas_dspr.c"))
        .file(cblas_dir.join("src/cblas_dspr2.c"))
        .file(cblas_dir.join("src/cblas_dswap.c"))
        .file(cblas_dir.join("src/cblas_dsymm.c"))
        .file(cblas_dir.join("src/cblas_dsymv.c"))
        .file(cblas_dir.join("src/cblas_dsyr.c"))
        .file(cblas_dir.join("src/cblas_dsyr2.c"))
        .file(cblas_dir.join("src/cblas_dsyr2k.c"))
        .file(cblas_dir.join("src/cblas_dsyrk.c"))
        .file(cblas_dir.join("src/cblas_dtbmv.c"))
        .file(cblas_dir.join("src/cblas_dtbsv.c"))
        .file(cblas_dir.join("src/cblas_dtpmv.c"))
        .file(cblas_dir.join("src/cblas_dtpsv.c"))
        .file(cblas_dir.join("src/cblas_dtrmm.c"))
        .file(cblas_dir.join("src/cblas_dtrmv.c"))
        .file(cblas_dir.join("src/cblas_dtrsm.c"))
        .file(cblas_dir.join("src/cblas_dtrsv.c"))
        .file(cblas_dir.join("src/cblas_dzasum.c"))
        .file(cblas_dir.join("src/cblas_dznrm2.c"))
        .file(cblas_dir.join("src/cblas_globals.c"))
        .file(cblas_dir.join("src/cblas_icamax.c"))
        .file(cblas_dir.join("src/cblas_idamax.c"))
        .file(cblas_dir.join("src/cblas_isamax.c"))
        .file(cblas_dir.join("src/cblas_izamax.c"))
        .file(cblas_dir.join("src/cblas_sasum.c"))
        .file(cblas_dir.join("src/cblas_saxpy.c"))
        .file(cblas_dir.join("src/cblas_scasum.c"))
        .file(cblas_dir.join("src/cblas_scnrm2.c"))
        .file(cblas_dir.join("src/cblas_scopy.c"))
        .file(cblas_dir.join("src/cblas_sdot.c"))
        .file(cblas_dir.join("src/cblas_sdsdot.c"))
        .file(cblas_dir.join("src/cblas_sgbmv.c"))
        .file(cblas_dir.join("src/cblas_sgemm.c"))
        .file(cblas_dir.join("src/cblas_sgemv.c"))
        .file(cblas_dir.join("src/cblas_sger.c"))
        .file(cblas_dir.join("src/cblas_snrm2.c"))
        .file(cblas_dir.join("src/cblas_srot.c"))
        .file(cblas_dir.join("src/cblas_srotg.c"))
        .file(cblas_dir.join("src/cblas_srotm.c"))
        .file(cblas_dir.join("src/cblas_srotmg.c"))
        .file(cblas_dir.join("src/cblas_ssbmv.c"))
        .file(cblas_dir.join("src/cblas_sscal.c"))
        .file(cblas_dir.join("src/cblas_sspmv.c"))
        .file(cblas_dir.join("src/cblas_sspr.c"))
        .file(cblas_dir.join("src/cblas_sspr2.c"))
        .file(cblas_dir.join("src/cblas_sswap.c"))
        .file(cblas_dir.join("src/cblas_ssymm.c"))
        .file(cblas_dir.join("src/cblas_ssymv.c"))
        .file(cblas_dir.join("src/cblas_ssyr.c"))
        .file(cblas_dir.join("src/cblas_ssyr2.c"))
        .file(cblas_dir.join("src/cblas_ssyr2k.c"))
        .file(cblas_dir.join("src/cblas_ssyrk.c"))
        .file(cblas_dir.join("src/cblas_stbmv.c"))
        .file(cblas_dir.join("src/cblas_stbsv.c"))
        .file(cblas_dir.join("src/cblas_stpmv.c"))
        .file(cblas_dir.join("src/cblas_stpsv.c"))
        .file(cblas_dir.join("src/cblas_strmm.c"))
        .file(cblas_dir.join("src/cblas_strmv.c"))
        .file(cblas_dir.join("src/cblas_strsm.c"))
        .file(cblas_dir.join("src/cblas_strsv.c"))
        .file(cblas_dir.join("src/cblas_xerbla.c"))
        .file(cblas_dir.join("src/cblas_zaxpy.c"))
        .file(cblas_dir.join("src/cblas_zcopy.c"))
        .file(cblas_dir.join("src/cblas_zdotc_sub.c"))
        .file(cblas_dir.join("src/cblas_zdotu_sub.c"))
        .file(cblas_dir.join("src/cblas_zdscal.c"))
        .file(cblas_dir.join("src/cblas_zgbmv.c"))
        .file(cblas_dir.join("src/cblas_zgemm.c"))
        .file(cblas_dir.join("src/cblas_zgemv.c"))
        .file(cblas_dir.join("src/cblas_zgerc.c"))
        .file(cblas_dir.join("src/cblas_zgeru.c"))
        .file(cblas_dir.join("src/cblas_zhbmv.c"))
        .file(cblas_dir.join("src/cblas_zhemm.c"))
        .file(cblas_dir.join("src/cblas_zhemv.c"))
        .file(cblas_dir.join("src/cblas_zher.c"))
        .file(cblas_dir.join("src/cblas_zher2.c"))
        .file(cblas_dir.join("src/cblas_zher2k.c"))
        .file(cblas_dir.join("src/cblas_zherk.c"))
        .file(cblas_dir.join("src/cblas_zhpmv.c"))
        .file(cblas_dir.join("src/cblas_zhpr.c"))
        .file(cblas_dir.join("src/cblas_zhpr2.c"))
        .file(cblas_dir.join("src/cblas_zscal.c"))
        .file(cblas_dir.join("src/cblas_zswap.c"))
        .file(cblas_dir.join("src/cblas_zsymm.c"))
        .file(cblas_dir.join("src/cblas_zsyr2k.c"))
        .file(cblas_dir.join("src/cblas_zsyrk.c"))
        .file(cblas_dir.join("src/cblas_ztbmv.c"))
        .file(cblas_dir.join("src/cblas_ztbsv.c"))
        .file(cblas_dir.join("src/cblas_ztpmv.c"))
        .file(cblas_dir.join("src/cblas_ztpsv.c"))
        .file(cblas_dir.join("src/cblas_ztrmm.c"))
        .file(cblas_dir.join("src/cblas_ztrmv.c"))
        .file(cblas_dir.join("src/cblas_ztrsm.c"))
        .file(cblas_dir.join("src/cblas_ztrsv.c"))

        // TODO build fortran files aswell 

        .compile("libcblas");
}
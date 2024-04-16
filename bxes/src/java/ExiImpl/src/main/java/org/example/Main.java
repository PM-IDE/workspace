package org.example;

import com.siemens.ct.exi.core.CodingMode;
import com.siemens.ct.exi.core.EXIFactory;
import com.siemens.ct.exi.core.EncodingOptions;
import com.siemens.ct.exi.core.FidelityOptions;
import com.siemens.ct.exi.core.helpers.DefaultEXIFactory;
import com.siemens.ct.exi.main.api.sax.EXIResult;
import com.siemens.ct.exi.main.api.sax.EXISource;
import org.xml.sax.XMLReader;
import org.xml.sax.helpers.XMLReaderFactory;

import java.io.FileOutputStream;
import java.io.OutputStream;

public class Main {
    public static void main(String[] args) {
        try {
            EXIFactory exiFactory = DefaultEXIFactory.newInstance();
            exiFactory.setCodingMode(CodingMode.COMPRESSION);

            var fidelityOptions = FidelityOptions.createDefault();
            fidelityOptions.setFidelity(FidelityOptions.FEATURE_LEXICAL_VALUE, true);
            exiFactory.setFidelityOptions(fidelityOptions);

            var options = EncodingOptions.createDefault();
            options.setOption(EncodingOptions.INCLUDE_SCHEMA_ID, true);

            exiFactory.setEncodingOptions(options);

            String fileEXI = args[1];
            OutputStream osEXI = new FileOutputStream(fileEXI);
            EXIResult exiResult = new EXIResult(exiFactory);
            exiResult.setOutputStream(osEXI);
            XMLReader xmlReader = XMLReaderFactory.createXMLReader();
            xmlReader.setContentHandler( exiResult.getHandler() );
            xmlReader.parse(args[0]);
            osEXI.close();
        }
        catch (Exception ex) {
            System.out.println(ex.getMessage());
        }
    }
}
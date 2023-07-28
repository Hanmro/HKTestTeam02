<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Community Powered Test Automation     _2410db</name>
   <tag></tag>
   <elementGuidId>20c1b93d-d49a-4189-9531-b9d76358fa9d</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>08982712-be78-4933-93ce-c1665f88b775</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
        
            
        
    
    
    
        
            Community Powered Test Automation
            TestProject simplifies and enhances the use of leading open source frameworks Selenium &amp; Appium. Driven by a community of passionate developers that create and share add-ons for Mobile, Web and API testing. All available for FREE!
	        
		        FREE SIGN UP
		        
			        Watch demos:
			        
				        
					        
					        Web
				        
				        
					        
					        Mobile
				        
			        
		        
	        
        
    
    
	    
		    TestProject Example page
		    This is the TestProject playground website. Feel free to play around it :)
		    
			    
				    Full Name:
				    
					    
						    
					    
					    
					    Please provide your full name
				    
			    
			    
				    Password:
				    
					    
						    
					    
					    
					    Hint: password is 12345
					    Password is invalid
				    
			    
			    
				    
					    Login
				    
			    
		    
	    
	    
		    TestProject Example page
		    Hello hkit, let's complete the test form:
		    
			    
				    Country:
				    
					    AfghanistanAlbaniaAlgeriaAmerican SamoaAndorraAngolaAnguillaAntarcticaAntigua and BarbudaArgentinaArmeniaArubaAustraliaAustriaAzerbaijanBahrainBangladeshBarbadosBelarusBelgiumBelizeBeninBermudaBhutanBoliviaBosnia and HerzegovinaBotswanaBouvet IslandBrazilBritish Indian Ocean TerritoryBritish Virgin IslandsBruneiBulgariaBurkina FasoBurundiCôte d'IvoireCambodiaCameroonCanadaCape VerdeCayman IslandsCentral African RepublicChadChileChinaChristmas IslandCocos (Keeling) IslandsColombiaComorosCongoCook IslandsCosta RicaCroatiaCubaCyprusCzech RepublicDemocratic Republic of the CongoDenmarkDjiboutiDominicaDominican RepublicEast TimorEcuadorEgyptEl SalvadorEquatorial GuineaEritreaEstoniaEthiopiaFaeroe IslandsFalkland IslandsFijiFinlandFormer Yugoslav Republic of MacedoniaFranceFrance, MetropolitanFrench GuianaFrench PolynesiaFrench Southern TerritoriesGabonGeorgiaGermanyGhanaGibraltarGreeceGreenlandGrenadaGuadeloupeGuamGuatemalaGuineaGuinea-BissauGuyanaHaitiHeard and Mc Donald IslandsHondurasHong KongHungaryIcelandIndiaIndonesiaIranIraqIrelandIsraelItalyJamaicaJapanJordanKazakhstanKenyaKiribatiKuwaitKyrgyzstanLaosLatviaLebanonLesothoLiberiaLibyaLiechtensteinLithuaniaLuxembourgMacauMadagascarMalawiMalaysiaMaldivesMaliMaltaMarshall IslandsMartiniqueMauritaniaMauritiusMayotteMexicoMicronesiaMoldovaMonacoMongoliaMontenegroMontserratMoroccoMozambiqueMyanmarNamibiaNauruNepalNetherlandsNetherlands AntillesNew CaledoniaNew ZealandNicaraguaNigerNigeriaNiueNorfolk IslandNorth KoreaNorthern MarianasNorwayOmanPakistanPalauPalestinePanamaPapua New GuineaParaguayPeruPhilippinesPitcairn IslandsPolandPortugalPuerto RicoQatarReunionRomaniaRussiaRwandaSão Tomé and PríncipeSaint HelenaSt. Pierre and MiquelonSaint Kitts and NevisSaint LuciaSaint Vincent and the GrenadinesSamoaSan MarinoSaudi ArabiaSenegalSerbiaSeychellesSierra LeoneSingaporeSlovakiaSloveniaSolomon IslandsSomaliaSouth AfricaSouth Georgia and the South Sandwich IslandsSouth KoreaSpainSri LankaSudanSurinameSvalbard and Jan Mayen IslandsSwazilandSwedenSwitzerlandSyriaTaiwanTajikistanTanzaniaThailandThe BahamasThe GambiaTogoTokelauTongaTrinidad and TobagoTunisiaTurkeyTurkmenistanTurks and Caicos IslandsTuvaluUS Virgin IslandsUgandaUkraineUnited Arab EmiratesUnited KingdomUnited StatesUnited States Minor Outlying IslandsUruguayUzbekistanVanuatuVatican CityVenezuelaVietnamWallis and Futuna IslandsWestern SaharaYemenZambiaZimbabwe
					    Please choose your country
				    
			    
			    
				    Address:
				    
					    
					    Please enter your address
				    
			    
			    
				    Email:
				    
					    
					    Please enter a valid email
				    
			    
			    
				    Phone:
				    
					    
					    Please enter a valid phone number
				    
			    
			    
				    Saved
			    
			    
				    
					    Save
				    
				    
					    Logout
				    
			    
		    
	    
    
    
	    
		    Try TestProject:
		    FREE SIGN UP
	    
	    
		    support@testproject.io
		    
			    
				    
			    
			    
				    
			    
		    
	    
    

    
        // Example starter JavaScript for disabling form submissions if there are invalid fields
        (function() {
            'use strict';
            window.addEventListener('load', function() {
                // Fetch all the forms we want to apply custom Bootstrap validation styles to
                var forms = document.getElementsByClassName('needs-validation');
                // Loop over them and prevent submission
                var validation = Array.prototype.filter.call(forms, function(form) {
                    form.addEventListener('submit', function(event) {
                        if (form.checkValidity() === false) {
                            event.preventDefault();
                            event.stopPropagation();
                        }
                        form.classList.add('was-validated');
                    }, false);
                });
            }, false);
        })();
    



id(&quot;saved&quot;)/h3[1]/span[@class=&quot;tp-saved&quot;]</value>
      <webElementGuid>ff5f9127-9e27-401f-89e0-9726a3ff93e0</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
      <webElementGuid>4d27259c-0462-44bd-be8e-07d0773ccf99</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>95916dd1-23e4-498d-ab9c-5b7d523b5159</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
    
        
            
        
    
    
    
        
            Community Powered Test Automation
            TestProject simplifies and enhances the use of leading open source frameworks Selenium &amp; Appium. Driven by a community of passionate developers that create and share add-ons for Mobile, Web and API testing. All available for FREE!
	        
		        FREE SIGN UP
		        
			        Watch demos:
			        
				        
					        
					        Web
				        
				        
					        
					        Mobile
				        
			        
		        
	        
        
    
    
	    
		    TestProject Example page
		    This is the TestProject playground website. Feel free to play around it :)
		    
			    
				    Full Name:
				    
					    
						    
					    
					    
					    Please provide your full name
				    
			    
			    
				    Password:
				    
					    
						    
					    
					    
					    Hint: password is 12345
					    Password is invalid
				    
			    
			    
				    
					    Login
				    
			    
		    
	    
	    
		    TestProject Example page
		    Hello hkit, let&quot; , &quot;'&quot; , &quot;s complete the test form:
		    
			    
				    Country:
				    
					    AfghanistanAlbaniaAlgeriaAmerican SamoaAndorraAngolaAnguillaAntarcticaAntigua and BarbudaArgentinaArmeniaArubaAustraliaAustriaAzerbaijanBahrainBangladeshBarbadosBelarusBelgiumBelizeBeninBermudaBhutanBoliviaBosnia and HerzegovinaBotswanaBouvet IslandBrazilBritish Indian Ocean TerritoryBritish Virgin IslandsBruneiBulgariaBurkina FasoBurundiCôte d&quot; , &quot;'&quot; , &quot;IvoireCambodiaCameroonCanadaCape VerdeCayman IslandsCentral African RepublicChadChileChinaChristmas IslandCocos (Keeling) IslandsColombiaComorosCongoCook IslandsCosta RicaCroatiaCubaCyprusCzech RepublicDemocratic Republic of the CongoDenmarkDjiboutiDominicaDominican RepublicEast TimorEcuadorEgyptEl SalvadorEquatorial GuineaEritreaEstoniaEthiopiaFaeroe IslandsFalkland IslandsFijiFinlandFormer Yugoslav Republic of MacedoniaFranceFrance, MetropolitanFrench GuianaFrench PolynesiaFrench Southern TerritoriesGabonGeorgiaGermanyGhanaGibraltarGreeceGreenlandGrenadaGuadeloupeGuamGuatemalaGuineaGuinea-BissauGuyanaHaitiHeard and Mc Donald IslandsHondurasHong KongHungaryIcelandIndiaIndonesiaIranIraqIrelandIsraelItalyJamaicaJapanJordanKazakhstanKenyaKiribatiKuwaitKyrgyzstanLaosLatviaLebanonLesothoLiberiaLibyaLiechtensteinLithuaniaLuxembourgMacauMadagascarMalawiMalaysiaMaldivesMaliMaltaMarshall IslandsMartiniqueMauritaniaMauritiusMayotteMexicoMicronesiaMoldovaMonacoMongoliaMontenegroMontserratMoroccoMozambiqueMyanmarNamibiaNauruNepalNetherlandsNetherlands AntillesNew CaledoniaNew ZealandNicaraguaNigerNigeriaNiueNorfolk IslandNorth KoreaNorthern MarianasNorwayOmanPakistanPalauPalestinePanamaPapua New GuineaParaguayPeruPhilippinesPitcairn IslandsPolandPortugalPuerto RicoQatarReunionRomaniaRussiaRwandaSão Tomé and PríncipeSaint HelenaSt. Pierre and MiquelonSaint Kitts and NevisSaint LuciaSaint Vincent and the GrenadinesSamoaSan MarinoSaudi ArabiaSenegalSerbiaSeychellesSierra LeoneSingaporeSlovakiaSloveniaSolomon IslandsSomaliaSouth AfricaSouth Georgia and the South Sandwich IslandsSouth KoreaSpainSri LankaSudanSurinameSvalbard and Jan Mayen IslandsSwazilandSwedenSwitzerlandSyriaTaiwanTajikistanTanzaniaThailandThe BahamasThe GambiaTogoTokelauTongaTrinidad and TobagoTunisiaTurkeyTurkmenistanTurks and Caicos IslandsTuvaluUS Virgin IslandsUgandaUkraineUnited Arab EmiratesUnited KingdomUnited StatesUnited States Minor Outlying IslandsUruguayUzbekistanVanuatuVatican CityVenezuelaVietnamWallis and Futuna IslandsWestern SaharaYemenZambiaZimbabwe
					    Please choose your country
				    
			    
			    
				    Address:
				    
					    
					    Please enter your address
				    
			    
			    
				    Email:
				    
					    
					    Please enter a valid email
				    
			    
			    
				    Phone:
				    
					    
					    Please enter a valid phone number
				    
			    
			    
				    Saved
			    
			    
				    
					    Save
				    
				    
					    Logout
				    
			    
		    
	    
    
    
	    
		    Try TestProject:
		    FREE SIGN UP
	    
	    
		    support@testproject.io
		    
			    
				    
			    
			    
				    
			    
		    
	    
    

    
        // Example starter JavaScript for disabling form submissions if there are invalid fields
        (function() {
            &quot; , &quot;'&quot; , &quot;use strict&quot; , &quot;'&quot; , &quot;;
            window.addEventListener(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, function() {
                // Fetch all the forms we want to apply custom Bootstrap validation styles to
                var forms = document.getElementsByClassName(&quot; , &quot;'&quot; , &quot;needs-validation&quot; , &quot;'&quot; , &quot;);
                // Loop over them and prevent submission
                var validation = Array.prototype.filter.call(forms, function(form) {
                    form.addEventListener(&quot; , &quot;'&quot; , &quot;submit&quot; , &quot;'&quot; , &quot;, function(event) {
                        if (form.checkValidity() === false) {
                            event.preventDefault();
                            event.stopPropagation();
                        }
                        form.classList.add(&quot; , &quot;'&quot; , &quot;was-validated&quot; , &quot;'&quot; , &quot;);
                    }, false);
                });
            }, false);
        })();
    



id(&quot;saved&quot;)/h3[1]/span[@class=&quot;tp-saved&quot;]&quot;) or . = concat(&quot;
    
        
            
        
    
    
    
        
            Community Powered Test Automation
            TestProject simplifies and enhances the use of leading open source frameworks Selenium &amp; Appium. Driven by a community of passionate developers that create and share add-ons for Mobile, Web and API testing. All available for FREE!
	        
		        FREE SIGN UP
		        
			        Watch demos:
			        
				        
					        
					        Web
				        
				        
					        
					        Mobile
				        
			        
		        
	        
        
    
    
	    
		    TestProject Example page
		    This is the TestProject playground website. Feel free to play around it :)
		    
			    
				    Full Name:
				    
					    
						    
					    
					    
					    Please provide your full name
				    
			    
			    
				    Password:
				    
					    
						    
					    
					    
					    Hint: password is 12345
					    Password is invalid
				    
			    
			    
				    
					    Login
				    
			    
		    
	    
	    
		    TestProject Example page
		    Hello hkit, let&quot; , &quot;'&quot; , &quot;s complete the test form:
		    
			    
				    Country:
				    
					    AfghanistanAlbaniaAlgeriaAmerican SamoaAndorraAngolaAnguillaAntarcticaAntigua and BarbudaArgentinaArmeniaArubaAustraliaAustriaAzerbaijanBahrainBangladeshBarbadosBelarusBelgiumBelizeBeninBermudaBhutanBoliviaBosnia and HerzegovinaBotswanaBouvet IslandBrazilBritish Indian Ocean TerritoryBritish Virgin IslandsBruneiBulgariaBurkina FasoBurundiCôte d&quot; , &quot;'&quot; , &quot;IvoireCambodiaCameroonCanadaCape VerdeCayman IslandsCentral African RepublicChadChileChinaChristmas IslandCocos (Keeling) IslandsColombiaComorosCongoCook IslandsCosta RicaCroatiaCubaCyprusCzech RepublicDemocratic Republic of the CongoDenmarkDjiboutiDominicaDominican RepublicEast TimorEcuadorEgyptEl SalvadorEquatorial GuineaEritreaEstoniaEthiopiaFaeroe IslandsFalkland IslandsFijiFinlandFormer Yugoslav Republic of MacedoniaFranceFrance, MetropolitanFrench GuianaFrench PolynesiaFrench Southern TerritoriesGabonGeorgiaGermanyGhanaGibraltarGreeceGreenlandGrenadaGuadeloupeGuamGuatemalaGuineaGuinea-BissauGuyanaHaitiHeard and Mc Donald IslandsHondurasHong KongHungaryIcelandIndiaIndonesiaIranIraqIrelandIsraelItalyJamaicaJapanJordanKazakhstanKenyaKiribatiKuwaitKyrgyzstanLaosLatviaLebanonLesothoLiberiaLibyaLiechtensteinLithuaniaLuxembourgMacauMadagascarMalawiMalaysiaMaldivesMaliMaltaMarshall IslandsMartiniqueMauritaniaMauritiusMayotteMexicoMicronesiaMoldovaMonacoMongoliaMontenegroMontserratMoroccoMozambiqueMyanmarNamibiaNauruNepalNetherlandsNetherlands AntillesNew CaledoniaNew ZealandNicaraguaNigerNigeriaNiueNorfolk IslandNorth KoreaNorthern MarianasNorwayOmanPakistanPalauPalestinePanamaPapua New GuineaParaguayPeruPhilippinesPitcairn IslandsPolandPortugalPuerto RicoQatarReunionRomaniaRussiaRwandaSão Tomé and PríncipeSaint HelenaSt. Pierre and MiquelonSaint Kitts and NevisSaint LuciaSaint Vincent and the GrenadinesSamoaSan MarinoSaudi ArabiaSenegalSerbiaSeychellesSierra LeoneSingaporeSlovakiaSloveniaSolomon IslandsSomaliaSouth AfricaSouth Georgia and the South Sandwich IslandsSouth KoreaSpainSri LankaSudanSurinameSvalbard and Jan Mayen IslandsSwazilandSwedenSwitzerlandSyriaTaiwanTajikistanTanzaniaThailandThe BahamasThe GambiaTogoTokelauTongaTrinidad and TobagoTunisiaTurkeyTurkmenistanTurks and Caicos IslandsTuvaluUS Virgin IslandsUgandaUkraineUnited Arab EmiratesUnited KingdomUnited StatesUnited States Minor Outlying IslandsUruguayUzbekistanVanuatuVatican CityVenezuelaVietnamWallis and Futuna IslandsWestern SaharaYemenZambiaZimbabwe
					    Please choose your country
				    
			    
			    
				    Address:
				    
					    
					    Please enter your address
				    
			    
			    
				    Email:
				    
					    
					    Please enter a valid email
				    
			    
			    
				    Phone:
				    
					    
					    Please enter a valid phone number
				    
			    
			    
				    Saved
			    
			    
				    
					    Save
				    
				    
					    Logout
				    
			    
		    
	    
    
    
	    
		    Try TestProject:
		    FREE SIGN UP
	    
	    
		    support@testproject.io
		    
			    
				    
			    
			    
				    
			    
		    
	    
    

    
        // Example starter JavaScript for disabling form submissions if there are invalid fields
        (function() {
            &quot; , &quot;'&quot; , &quot;use strict&quot; , &quot;'&quot; , &quot;;
            window.addEventListener(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, function() {
                // Fetch all the forms we want to apply custom Bootstrap validation styles to
                var forms = document.getElementsByClassName(&quot; , &quot;'&quot; , &quot;needs-validation&quot; , &quot;'&quot; , &quot;);
                // Loop over them and prevent submission
                var validation = Array.prototype.filter.call(forms, function(form) {
                    form.addEventListener(&quot; , &quot;'&quot; , &quot;submit&quot; , &quot;'&quot; , &quot;, function(event) {
                        if (form.checkValidity() === false) {
                            event.preventDefault();
                            event.stopPropagation();
                        }
                        form.classList.add(&quot; , &quot;'&quot; , &quot;was-validated&quot; , &quot;'&quot; , &quot;);
                    }, false);
                });
            }, false);
        })();
    



id(&quot;saved&quot;)/h3[1]/span[@class=&quot;tp-saved&quot;]&quot;))]</value>
      <webElementGuid>73f4aa3a-8788-423c-b622-25b520380f4c</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
